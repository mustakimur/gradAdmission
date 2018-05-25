function facts_app() {
    // a hashmap from property names to attributes
    var fields = {};
    fields["emp_id"] = { tag: "Employee ID", can_change: false, convert: parseInt, long: false };
    fields["applicant_id"] = { tag: "Applicant ID", can_change: false, convert: parseInt, long: false };
    fields["name"] = { tag: "Name", can_change: false, convert: null, long: false };
    fields["dob"] = { tag: "Date of Birth", can_change: false, convert: null, long: false };
    fields["gender"] = { tag: "Gender", can_change: false, convert: null, long: false };
    fields["country"] = { tag: "Country", can_change: false, convert: null, long: true };
    fields["program"] = { tag: "Program", can_change: false, convert: null, long: false };
    fields["degree"] = { tag: "Degree", can_change: false, convert: null, long: false };
    fields["interests"] = { tag: "Interests", can_change: true, convert: null, long: true };
    fields["ug_university"] = { tag: "UG University", can_change: true, convert: null, long: true };
    fields["ug_major"] = { tag: "UG Major", can_change: true, convert: null, long: true };
    fields["ug_degree"] = { tag: "UG Degree", can_change: true, convert: null, long: false };
    fields["ug_gpa"] = { tag: "UG GPA", can_change: true, convert: parseFloat, long: false };
    fields["grad_university"] = { tag: "Grad University", can_change: true, convert: null, long: true };
    fields["grad_major"] = { tag: "Grad Major", can_change: true, convert: null, long: true };
    fields["grad_degree"] = { tag: "Grad Degree", can_change: true, convert: null, long: false };
    fields["grad_gpa"] = { tag: "Grad GPA", can_change: true, convert: parseFloat, long: false };
    fields["toefl_ielts"] = { tag: "TOEFL/IELTS", can_change: true, convert: parseInt, long: false };
    fields["gre"] = { tag: "GRE", can_change: true, convert: null, long: false };
    fields["decision"] = { tag: "Decision", can_change: true, convert: null, long: true };
    fields["advisor"] = { tag: "Advisor", can_change: true, convert: null, long: false };
    fields["assistantship"] = { tag: "Assistantship", can_change: true, convert: null, long: false };
    fields["fte"] = { tag: "FTE", can_change: true, convert: parseFloat, long: false };
    fields["yearly_amount"] = { tag: "Salary", can_change: true, convert: parseInt, long: false };

    return fields;
}

function toast(msg) {
    // Get the snackbar DIV
    var t = document.getElementById("snackbar");

    // Add the "show" class to DIV
    t.className = "show";
    t.innerHTML = msg;

    // After 3 seconds, remove the show class from DIV
    setTimeout(function(){ t.className = t.className.replace("show", ""); }, 3500);
}


/* function create_tbl(){
     function create_info() {
                function gen_td(field) {
                    if (fields[field].long) {
                        return '<td class="td_info">' + fields[field].tag + ':</td>' +
                            '<td class="td_info" colspan="3"> ' + field + '</td>';

                    } else {
                        return '<td class="td_info">' + fields[field].tag + ':</td>' +
                            '<td class="td_info"> ' + field + '</td>';
                    }
                }

                function gen_link(field) {
                    return '<td class="td_info">' + field + ':</td>' +
                        '<td class="td_info"> ' + '</td>';
                }

                // `row_data` is the original data object for the row
                return '<table id="tbl_info">' +
                    '<td colspan="4"><b>Basic Information:</b></td></tr><tr>' +
                    gen_td("emp_id") + gen_td("applicant_id") +
                    '</tr>' + '<tr>' +
                    gen_td("name") + gen_td("gender") +
                    '</tr>' + '<tr>' +
                    gen_td("country") +
                    '</tr>' + '<tr>' +
                    gen_td("program") +
                    gen_td("degree") +
                    '</tr>' + '<tr>' +
                    gen_td("interests") +
                    '</tr>' + '<tr>' +
                    gen_td("ug_university") +
                    '</tr>' + '<tr>' +
                    gen_td("ug_major") +
                    '</tr>' + '<tr>' +
                    gen_td("ug_degree") + gen_td("ug_gpa") +
                    '</tr>' + '<tr>' +
                    gen_td("grad_university") +
                    '</tr>' + '<tr>' +
                    gen_td("grad_major") +
                    '</tr>' + '<tr>' +
                    gen_td("grad_degree") + gen_td("grad_gpa") +
                    '</tr>' + '<tr>' +
                    gen_td("toefl_ielts") + gen_td("gre") +
                    '</tr>' + '<tr>' +
                    gen_td("decision") +
                    '</tr>' + '<tr>' +
                    gen_td("advisor") + gen_td("assistantship") +
                    '</tr>' + '<tr>' +
                    gen_td("fte") + gen_td("yearly_amount") +
                    '</tr>' + '<tr>' +
                    '<td colspan="4"><b>Supporting Materials:</b></td></tr><tr>' +
                    gen_link("resume") + gen_link("Statement") +
                    '</tr>' + '<tr>' +
                    gen_link("ug_transcript") + gen_link("grad_transcript") +
                    '</tr>' + '<tr>' +
                    gen_link("toefl") + gen_link("gre") +
                    '</tr>' + '<tr>' +
                    gen_link("letter1") + gen_link("letter2") +
                    '</tr>' + '<tr>' +
                    gen_link("letter3") + gen_link("letter4") +
                    '</tr>' + '<tr>' +
                    gen_link("other1") + gen_link("other2")
                    ;
                // +
                //'<td></td><td> <button type="button" class="btn btn-primary" id="btn_' +'">Update</button>' +
                //'</table>';
            }

            $("div.info").html(create_info());
} */