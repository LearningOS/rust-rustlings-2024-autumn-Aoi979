pub trait Grade {
    fn as_string(&self) -> String;
}

impl Grade for f32 {
    fn as_string(&self) -> String {
        self.to_string()
    }
}

impl Grade for String {
    fn as_string(&self) -> String {
        self.clone()
    }
}

pub struct ReportCard<T: Grade> {
    pub grade: T,
    pub student_name: String,
    pub student_age: u8,
}

impl<T: Grade> ReportCard<T> {
    pub fn print(&self) -> String {
        format!("{} ({}) - achieved a grade of {}",
                &self.student_name, &self.student_age, self.grade.as_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_numeric_report_card() {
        let report_card = ReportCard {
            grade: 2.1,
            student_name: "Tom Wriggle".to_string(),
            student_age: 12,
        };
        assert_eq!(
            report_card.print(),
            "Tom Wriggle (12) - achieved a grade of 2.1"
        );
    }

    #[test]
    fn generate_alphabetic_report_card() {
        let report_card = ReportCard {
            grade: "A+".to_string(),
            student_name: "Gary Plotter".to_string(),
            student_age: 11,
        };
        assert_eq!(
            report_card.print(),
            "Gary Plotter (11) - achieved a grade of A+"
        );
    }
}
