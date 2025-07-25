use uuid::Uuid;
use chrono::{NaiveDateTime, NaiveDate, Utc, DateTime};
use serde::{Serialize, Deserialize};
use std::str::FromStr;

#[cfg(feature = "database")]
use sqlx::{FromRow, Type};

// ============= SYSTEM USER MANAGEMENT ===============

/// Role of a system user
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "database", derive(sqlx::Type))]
#[cfg_attr(feature = "database", sqlx(rename_all = "lowercase"))]
pub enum UserRole {
    Admin,
    CommitteeMember,
    Headteacher,
    DataEntry,
    Staff,
    Teacher,
}

/// Represents a person's full name
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonName {
    pub first_name: String,
    pub surname: String,
    pub other_names: Option<String>,
}

/// A system login user (database representation)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "database", derive(FromRow))]
pub struct UserAccount {
    pub id: Uuid,
    pub username: String,
    pub password_hash: String,
    pub role: UserRole,
    pub is_active: bool,
    pub first_name: String,
    pub surname: String,
    pub other_names: Option<String>,
}

/// User information for UI display
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfile {
    pub id: Uuid,
    pub username: String,
    pub name: PersonName,
    pub role: UserRole,
}

// Type alias for convenience in the new UI
pub type User = UserProfile;


impl From<UserAccount> for UserProfile {
    fn from(account: UserAccount) -> Self {
        Self {
            id: account.id,
            username: account.username,
            name: PersonName {
                first_name: account.first_name,
                surname: account.surname,
                other_names: account.other_names,
            },
            role: account.role,
        }
    }
}

impl From<UserResponse> for UserProfile {
    fn from(account: UserResponse) -> Self {
        Self {
            id: Uuid::from_str(&account.id).expect("Invalid UUID from server"),
            username: account.username,
            name: PersonName {
                first_name: account.first_name,
                surname: account.surname,
                other_names: account.other_names,
            },
            role: account.role,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserResponse {
    pub id: String,
    pub username: String,
    pub role: UserRole,
    pub is_active: bool,
    pub first_name: String,
    pub surname: String,
    pub other_names: Option<String>,
}

// ============= BASIC ENUMS ===============

/// Gender enum
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "database", derive(sqlx::Type))]
#[cfg_attr(feature = "database", sqlx(rename_all = "lowercase"))]
pub enum Gender {
    Male,
    Female,
}

impl Gender {
    pub const ALL: [Gender; 2] = [Gender::Male, Gender::Female]; 

    pub fn excel_options() -> &'static str {
        "Male,Female"
    }
}

impl std::fmt::Display for Gender {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self) 
    }
}

/// Academic class/level
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "database", derive(sqlx::Type))]
#[cfg_attr(feature = "database", sqlx(type_name = "class_level"))]
#[cfg_attr(feature = "database", sqlx(rename_all = "snake_case"))]
pub enum ClassLevel {
    LowerSecondaryYear8,
    LowerSecondaryYear9,
    IGCSE1,
    IGCSE2,
    WASSCE1,
    WASSCE2,
    WASSCE3,
    ALevel1,
    ALevel2,
}

impl ClassLevel {
    pub const ALL: [ClassLevel; 9] = [ 
        ClassLevel::LowerSecondaryYear8,
        ClassLevel::LowerSecondaryYear9,
        ClassLevel::IGCSE1,
        ClassLevel::IGCSE2,
        ClassLevel::WASSCE1,
        ClassLevel::WASSCE2,
        ClassLevel::WASSCE3,
        ClassLevel::ALevel1,
        ClassLevel::ALevel2,
    ];

    pub fn excel_options() -> &'static str {
        "LowerSecondaryYear8,LowerSecondaryYear9,IGCSE1,IGCSE2,WASSCE1,WASSCE2,WASSCE3,ALevel1,ALevel2"
    }
}

impl std::fmt::Display for ClassLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl FromStr for ClassLevel {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "LowerSecondaryYear8" => Ok(ClassLevel::LowerSecondaryYear8),
            "LowerSecondaryYear9" => Ok(ClassLevel::LowerSecondaryYear9),
            "IGCSE1" => Ok(ClassLevel::IGCSE1),
            "IGCSE2" => Ok(ClassLevel::IGCSE2),
            "WASSCE1" => Ok(ClassLevel::WASSCE1),
            "WASSCE2" => Ok(ClassLevel::WASSCE2),
            "WASSCE3" => Ok(ClassLevel::WASSCE3),
            "ALevel1" => Ok(ClassLevel::ALevel1),
            "ALevel2" => Ok(ClassLevel::ALevel2),
            _ => Err(()),
        }
    }
}

/// Education stream (syllabus)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "database", derive(sqlx::Type))]
#[cfg_attr(feature = "database", sqlx(rename_all = "PascalCase"))]
pub enum Stream {
    CambridgeIGCSE,
    CambridgeLowerSecondary,
    CambridgeALevel,
    WASSCE,
}

/// Status of teacher's participation in the tuition program
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "database", derive(sqlx::Type))]
#[cfg_attr(feature = "database", sqlx(rename_all = "PascalCase"))]
pub enum ParticipationStatus {
    Participating,
    NotParticipating,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "database", derive(sqlx::Type))]
#[cfg_attr(feature = "database", sqlx(rename_all = "PascalCase"))]
pub enum SubjectName {
    English,
    Mathematics,
    Science,
    ICT,
    SocialStudies,
    French,
    Physics,
    Chemistry,
    Biology,
    Economics,
    Business,
    Geography,
    CoreMath,
    ElectiveMath,
    FurtherMath,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "database", derive(sqlx::Type))]
#[cfg_attr(feature = "database", sqlx(rename_all = "lowercase"))]
pub enum AcademicYear {
    Year2024_2025,
    Year2025_2026,
    Year2026_2027,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "database", derive(sqlx::Type))]
#[cfg_attr(feature = "database", sqlx(rename_all = "lowercase"))]
pub enum SupportStaffRole {
    Janitor,
    Administrator,
    Headteacher,
}

// ============= DATABASE ROWS ===============

/// Row from the `students` table
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "database", derive(FromRow))]
pub struct StudentRow {
    pub id: Uuid,
    pub first_name: String,
    pub surname: String,
    pub other_names: Option<String>,
    pub gender: Gender,
    pub class_level: ClassLevel, 
    pub is_active: bool,
}

/// Row from the `subjects` table
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "database", derive(FromRow))]
pub struct SubjectRow {
    pub id: Uuid,
    pub name: SubjectName,
    pub stream: Stream,
}

/// Row from the `conducted_periods` table
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "database", derive(FromRow))]
pub struct ConductedPeriodRow {
    pub id: Uuid,
    pub date: NaiveDateTime,
    pub class_level: ClassLevel,
    pub subject_id: Uuid,
    pub teacher_id: Uuid,
    pub term_id: Uuid,
    pub week_of_term: i32,
    pub was_conducted: bool,
    pub notes: Option<String>,
}

/// Row from the `terms` table
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "database", derive(FromRow))]
pub struct TermRow {
    pub id: Uuid,
    pub name: String,
    pub academic_year: AcademicYear,
    pub start_date: NaiveDateTime,
    pub end_date: NaiveDateTime,
    pub is_active: bool,
}

/// Row from the `teachers` table
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "database", derive(FromRow))]
pub struct TeacherRow {
    pub id: Uuid,
    pub first_name: String,
    pub surname: String,
    pub other_names: Option<String>,
    pub status: ParticipationStatus,
    pub is_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "database", derive(FromRow))]
pub struct SupportStaffRow {
    pub id: Uuid,
    pub first_name: String,
    pub surname: String,
    pub other_names: Option<String>,
    pub role: SupportStaffRole,
    pub is_active: bool,
}

// ============= DOMAIN MODELS ===============

/// Represents a student in the app state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Student {
    pub id: Uuid,
    pub name: PersonName,
    pub gender: Gender,
    pub class_level: ClassLevel,
    pub is_active: bool,
    pub fee_amount: f64,
    pub payment_status: PaymentStatus,
}

impl From<StudentRow> for Student {
    fn from(row: StudentRow) -> Self {
        Self {
            id: row.id,
            name: PersonName {
                first_name: row.first_name,
                surname: row.surname,
                other_names: row.other_names,
            },
            gender: row.gender,
            class_level: row.class_level,
            is_active: row.is_active,
            // Default values for new fields
            fee_amount: 0.0, 
            payment_status: PaymentStatus::NotPaid,
        }
    }
}

/// Represents a teacher
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Teacher {
    pub id: Uuid,
    pub name: PersonName,
    pub status: ParticipationStatus,
    pub is_active: bool,
}

impl From<TeacherRow> for Teacher {
    fn from(row: TeacherRow) -> Self {
        Self {
            id: row.id,
            name: PersonName {
                first_name: row.first_name,
                surname: row.surname,
                other_names: row.other_names,
            },
            status: row.status,
            is_active: row.is_active,
        }
    }
}

/// Support staff member
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupportStaff {
    pub id: Uuid,
    pub name: PersonName,
    pub role: SupportStaffRole,
    pub is_active: bool,
}

impl From<SupportStaffRow> for SupportStaff {
    fn from(row: SupportStaffRow) -> Self {
        Self {
            id: row.id,
            name: PersonName {
                first_name: row.first_name,
                surname: row.surname,
                other_names: row.other_names,
            },
            role: row.role,
            is_active: row.is_active,
        }
    }
}

/// Subject offered
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subject {
    pub id: Uuid,
    pub name: SubjectName,
    pub stream: Stream,
}

impl From<SubjectRow> for Subject {
    fn from(row: SubjectRow) -> Self {
        Self {
            id: row.id,
            name: row.name,
            stream: row.stream,
        }
    }
}

/// Term information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Term {
    pub id: Uuid,
    pub name: String,
    pub academic_year: AcademicYear,
    pub start_date: NaiveDateTime,
    pub end_date: NaiveDateTime,
    pub is_active: bool,
}

impl From<TermRow> for Term {
    fn from(row: TermRow) -> Self {
        Self {
            id: row.id,
            name: row.name,
            academic_year: row.academic_year,
            start_date: row.start_date,
            end_date: row.end_date,
            is_active: row.is_active,
        }
    }
}

/// Core unit of work done — a taught period
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConductedPeriod {
    pub id: Uuid,
    pub date: NaiveDateTime,
    pub class_level: ClassLevel,
    pub subject_id: Uuid,
    pub teacher_id: Uuid,
    pub term_id: Uuid,
    pub week_of_term: u8,
    pub was_conducted: bool,
    pub notes: Option<String>,
}

impl From<ConductedPeriodRow> for ConductedPeriod {
    fn from(row: ConductedPeriodRow) -> Self {
        Self {
            id: row.id,
            date: row.date,
            class_level: row.class_level,
            subject_id: row.subject_id,
            teacher_id: row.teacher_id,
            term_id: row.term_id,
            week_of_term: row.week_of_term as u8,
            was_conducted: row.was_conducted,
            notes: row.notes,
        }
    }
}

// ============= FINANCIAL & UI MODELS ===============

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PaymentStatus {
    Paid,
    Partial,
    NotPaid,
    Exempt,
}

impl std::fmt::Display for PaymentStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// UI-specific Payment model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Payment {
    pub id: String,
    pub student_id: String,
    pub amount: f64,
    pub method: String,
    pub description: String,
    pub status: PaymentStatus,
}

/// UI-specific TeachingPeriod model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeachingPeriod {
    pub id: String,
    pub subject: String,
    pub class: String,
    pub rate: f64,
    pub date: NaiveDate,
    pub start_time: u32,
    pub end_time: u32,
    pub teacher_name: String,
}

#[derive(Debug, Clone)]
pub struct RecentActivity {
    pub id: String,
    pub activity_type: ActivityType,
    pub description: String,
    pub timestamp: DateTime<Utc>,
    pub user_name: String,
}

#[derive(Debug, Clone)]
pub enum ActivityType {
    PaymentReceived,
    StudentAdded,
    TeachingPeriodCreated,
    UserCreated,
}

#[derive(Debug, Clone)]
pub struct PaymentSummary {
    pub total_expected: f64,
    pub total_received: f64,
    pub total_pending: f64,
    pub paid_count: usize,
    pub partial_count: usize,
    pub unpaid_count: usize,
    pub exempt_count: usize,
}

#[derive(Debug, Clone)]
pub struct TeacherEarnings {
    pub teacher_id: String,
    pub teacher_name: String,
    pub total_periods: usize,
    pub total_earnings: f64,
    pub share_percentage: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ReportType {
    ProjectedIncome,
    CollectionStatus,
    TeacherEarnings,
    StudentPayments,
}


// ============= ORIGINAL FINANCIAL MODELS ===============

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DbPayment {
    pub id: Uuid,
    pub student_id: Uuid,
    pub term_id: Uuid,
    pub amount_paid: f64,
    pub date_paid: NaiveDateTime,
    pub recorded_by: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstraintConfig {
    pub id: Uuid,
    pub name: String,         
    pub base_percentage: f64,
    pub admin_percentage: f64,
    pub support_staff_percentage: f64,
    pub max_periods_paid: Option<u32>,
    pub max_ratio: Option<f64>,    
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Disbursement {
    pub id: Uuid,
    pub term_id: Uuid,
    pub total_revenue: f64,
    pub admin_share: f64,
    pub support_staff_share: f64,
    pub teacher_base_share: f64,
    pub teacher_period_share: f64,
    pub constraint_id: Uuid,
    pub calculated_on: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeacherPayout {
    pub id: Uuid,
    pub disbursement_id: Uuid,
    pub teacher_id: Uuid,
    pub base_share: f64,
    pub period_share: f64,
    pub capped: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeacherPayoutItem {
    pub id: Uuid,
    pub payout_id: Uuid,
    pub conducted_period_id: Uuid,
    pub paid_amount: f64,
}


// ============= HTTP/API COMMS ===============

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateStudentPayload {
    pub first_name: String,
    pub surname: String,
    pub other_names: Option<String>,
    pub gender: Gender,
    pub class_level: ClassLevel,
}

#[derive(Deserialize, Serialize)]
pub struct UpdateStudentPayload {
    pub id: Uuid,
    pub first_name: String,
    pub surname: String,
    pub other_names: Option<String>,
    pub gender: Gender,
    pub class_level: ClassLevel,
}
