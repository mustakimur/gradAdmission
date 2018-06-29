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
    fields["term"] = { tag: "Admit Term", can_change: false, convert: null, long: false };
    fields["interests"] = { tag: "Interests", can_change: true, convert: null, long: true };
    fields["ug_university"] = { tag: "UG University", can_change: true, convert: null, long: true };
    fields["ug_major"] = { tag: "UG Major", can_change: true, convert: null, long: true };
    fields["ug_degree"] = {
        tag: "UG Degree",
        can_change: true,
        convert: null,
        long: false,
        select: ["B.Science", "B.Tech", "B.Arts","B.Engineering", "Bachelor", "None"]
    };
    fields["ug_gpa"] = { tag: "UG GPA", can_change: true, convert: parseFloat, long: false };
    fields["grad_university"] = { tag: "Grad University", can_change: true, convert: null, long: true };
    fields["grad_major"] = { tag: "Grad Major", can_change: true, convert: null, long: true };
    fields["grad_degree"] = {
        tag: "Grad Degree",
        can_change: true,
        convert: null,
        long: false,
        select: ["None", "M.Science", "M.Engineering", "M.Arts", "Master"]
    };
    fields["grad_gpa"] = { tag: "Grad GPA", can_change: true, convert: parseFloat, long: false };
    fields["toefl_ielts"] = { tag: "TOEFL/IELTS (Total/L/R/W/S)", can_change: true, convert: null, long: false };
    fields["gre"] = { tag: "GRE (V/Q/A)", can_change: true, convert: null, long: false };
    fields["decision"] = {
        tag: "Decision",
        can_change: true,
        convert: null,
        long: true,
        select: [
            "New",
            "Incomplete",
            "Ready to review",
            "Decn: Admit",
            "Decn: Admit/HP",
            "Decn: Admit/support",
            "Decn: Reject",
            "Ntfy: Admit",
            "Ntfy: Admit/HP",
            "Ntfy: Admit/support",
            "Ntfy: Reject",
            "Stud: Accepted",
            "Stud: Cancelled",
            "Stud: Denied",
            "Stud: Retracted"
        ]
    };
    fields["advisor"] = { tag: "Advisor", can_change: true, convert: null, long: false };
    fields["assistantship"] = {
        tag: "Assistantship",
        can_change: true,
        convert: null,
        long: false,
        select: ["None", "TA", "DRTA", "RA", "TA+RA", "SFS"]
    };
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
    setTimeout(function() {
        t.className = t.className.replace("show", "");
    }, 2500);
}

function upload(url, file, on_complete) {
    fetch(url, {
        method: "POST",
        body: file,
        credentials: "same-origin" //send the cookies also!
    }).then(function(response) {
        response.text().then(function(text) {
            on_complete(text);
        });
    });
}

function delete_file(url, on_complete) {
    fetch(url, {
        method: "DELETE",
        credentials: "same-origin" //send the cookies also!
    }).then(function(response) {
        response.text().then(function(text) {
            on_complete(text);
        });
    });
}
