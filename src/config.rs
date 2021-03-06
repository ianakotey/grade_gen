use std::collections::HashMap;


pub(crate) const ENROLLMENT_SHEET_NAME: &str = "Detailed Enrollment List";

/// First row containing header data from CAMU
/// Header row is currently row 9
pub(crate) const ENROLLMENT_HEADER_ROW: usize = 9;

/// Index of column containing Student Id data. i.e. Column B
pub(crate) const ENROLLMENT_STUD_ID_COL_INDEX: usize = 1;

pub(crate) const ENROLLMENT_STUD_NAME_COL_INDEX: usize = 2;

/// Last column in CAMU detailed enrollment report
/// Currently Column T which contains email data
pub(crate) const ENROLLMENT_EMAIL_COL_INDEX: usize = 19;


pub(crate) const GRADEBOOK_NON_GRADE_COL_COUNT: usize = 5;

pub(crate) const GRADEBOOK_CSV_DELIMITER: char = ',';

pub(crate) const GRADEBOOK_EMAIL_COL_INDEX: usize = 3;

pub(crate) type EnrollmentData = Vec<(String, String, String)>;

pub(crate) type GradeMap = HashMap<String, String>;
pub(crate) type GradebookEntry = (usize, GradeMap);
pub(crate) type Gradebook = HashMap<String, GradebookEntry>;

pub(crate) const WRITER_SHEET_NAME: &str = "Mark Upload";