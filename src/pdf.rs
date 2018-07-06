use lopdf::{Document, Object};

use std::collections::HashMap;
use std::path::PathBuf;
use std::process::Command;
use std::{env, fs};

#[derive(Debug)]
struct Section {
    title: String,
    start_pg: u32,
    end_pg: u32,
}

trait AsString {
    fn as_string(&self) -> Option<String>;
}

impl AsString for lopdf::Object {
    fn as_string(&self) -> Option<String> {
        if let Object::String(ref text, _) = self {
            Some(String::from_utf8_lossy(&text.to_vec()).to_string())
        } else {
            None
        }
    }
}

// the following is a fix for the messy output by slate
fn duplicate(title: &str) -> bool {
    let low = title.to_lowercase();

    if low.contains("auto") {
        false
    } else if low.contains("unofficial") {
        false
    } else {
        true
    }
}

fn rename(title: &str, title2cnt: &mut HashMap<String, u32>) -> String {
    let title = title.to_lowercase();

    let name;

    if title.contains("self-reported") {
        name = "Self-reported";
    } else if title.contains("unofficial") {
        name = "Unofficial";
    } else if title.contains("dashboard") {
        name = "Summary";
    } else if title.contains("reference") {
        name = "Letter";
    } else if title.contains("statement") {
        name = "Statement";
    } else if title.contains("transcript") {
        name = "Transcript";
    } else if title.contains("resume") {
        name = "Resume";
    } else if title.contains("auto") {
        name = "General";
    } else {
        name = "Other";
    }

    if !title2cnt.contains_key(name) {
        title2cnt.insert(name.to_string(), 1);
    } else {
        *(title2cnt.get_mut(name).unwrap()) += 1;
    }

    // println!("title: {}, name: {}", title, name);
    format!("{}{}.pdf", name, *title2cnt.get(name).unwrap())
}

// split pdf by outlines, 2nd level, returns a list of new pdf files
pub fn split_pdf(fname: &PathBuf) -> Option<String> {
    // get the parent
    let mut root = env::current_dir().ok()?;
    root.push("resources");

    let parent = fname.parent()?;

    if !parent.is_dir() {
        warn!("Cannot get path from {}", fname.display());
        return None;
    }

    // The sections of the pdf file
    let mut sections: Vec<Section> = vec![];

    // open the file and save the object id to page number mapping
    let doc = Document::load(fname).ok()?;

    let pgs = doc.get_pages();
    let mut id2pg = HashMap::new();

    let mut max_pg = 0;

    // key is &u32, value is &object_id
    for (k, v) in pgs.iter() {
        id2pg.insert(v, *k);

        if *k > max_pg {
            max_pg = *k;
        }
    }

    info!("Total # of pages: {}", max_pg);

    // get the table of content
    let toc_id = doc.catalog()?.get("Outlines")?.as_reference()?;
    let toc = doc.get_object(toc_id)?.as_dict()?;

    // the first level outline, we use 2nd level of outline
    let level1 = doc
        .get_object(toc.get("First")?.as_reference()?)?
        .as_dict()?;

    //println!("{}", level1.get("Title")?.as_string()?);

    // the first 2nd-level outline
    let first = doc
        .get_object(level1.get("First")?.as_reference()?)?
        .as_dict()?;

    let mut l2 = first;

    // find all the level 2 outlines
    loop {
        let title = l2.get("Title")?.as_string()?;
        let dest = l2.get("Dest")?.as_array()?;
        let pg = *id2pg.get(&dest[0].as_reference()?)?;

        let sec = Section {
            title: title,
            start_pg: pg,
            end_pg: 0,
        };

        info!("Section in pdf: {:?}", sec);
        sections.push(sec);

        let next = l2.get("Next");
        if next.is_none() {
            break;
        }

        let next = doc.get_object(next?.as_reference()?)?.as_dict()?;
        l2 = next;

        //println!("next: {:?}", l2);
    }

    let len = sections.len();

    if len < 1 {
        return None;
    }

    // fix the end_pg
    for i in 0..len - 1 {
        sections[i].end_pg = sections[i + 1].start_pg - 1;
    }

    sections[len - 1].end_pg = max_pg;

    // Combine the sections with the same title
    let mut title2ranges = HashMap::new();

    for s in &sections {
        if !title2ranges.contains_key(&s.title) {
            title2ranges.insert(&s.title, vec![(s.start_pg, s.end_pg)]);
        } else {
            title2ranges.get_mut(&s.title)?.push((s.start_pg, s.end_pg));
        }
    }

    //println!("{:?}", title2ranges);

    // separate the original pdf file into single-page pdfs
    Command::new("pdfseparate")
        .arg("-f")
        .arg("1")
        .arg("-l")
        .arg(max_pg.to_string())
        .arg(fname)
        .arg(parent.join("separated%d.pdf"))
        .output()
        .ok()?;

    let mut title2cnt = HashMap::new();

    // now, unite the files to create new files
    for tr in &title2ranges {
        let mut cmd = Command::new("pdfunite");
        let ranges = tr.1;

        for k in 0..ranges.len() {
            if k > 0 && duplicate(tr.0) {
                cmd.arg(root.join("alert.pdf"));
            }

            for i in ranges[k].0..ranges[k].1 + 1 {
                cmd.arg(parent.join(format!("separated{}.pdf", i)));
            }
        }

        // rename the files and append the path
        let title = rename(tr.0, &mut title2cnt);

        cmd.arg(parent.join(title));
        let output = cmd.output().ok()?;
        //println!("status: {}", output.status);
        //println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
        info!("stderr: {}", String::from_utf8_lossy(&output.stderr));
    }

    // now, delete the tmp files
    for i in 1..max_pg + 1 {
        fs::remove_file(parent.join(format!("separated{}.pdf", i))).ok();
    }

    Some("Success".to_string())
}
