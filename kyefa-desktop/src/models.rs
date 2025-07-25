use uuid::Uuid;
use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};

// =========== USER ============

/// Role of a system user
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UserRole {
    Admin,
    CommitteeMember,
    Headteacher,
    DataEntry,
}

/// A system login user
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserAccount {
    pub id: Uuid,
    pub username: String,
    pub name: PersonName,
    pub password_hash: String,
    pub role: UserRole,
    pub is_active: bool,
}

// =========== INTERNAL APP STATE ============

/// Represents a person’s full name
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonName {
    pub first_name: String,
    pub surname: String,
    pub other_names: Option<String>,
}

/// Gender enum
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Gender {
    Male,
    Female,
}

/// Academic class/level
#[derive(Debug, Clone, Serialize, Deserialize)]
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

/// Represents a student in the app state
#[derive(Debug, Clone)]
pub struct Student {
    pub id: Uuid,
    pub name: PersonName,
    pub gender: Gender,
    pub class_level: ClassLevel,
    pub is_active: bool,
}

/// Status of teacher’s participation in the tuition program
#[derive(Debug, Clone)]
pub enum ParticipationStatus {
    Participating,
    NotParticipating,
}

/// Represents a teacher
#[derive(Debug, Clone)]
pub struct Teacher {
    pub id: Uuid,
    pub name: PersonName,
    pub status: ParticipationStatus,
    pub is_active: bool,
}

#[derive(Debug, Clone)]
enum SupportStaffRole {
    Janitor,
    Administrator,
    Headteacher,
}

#[derive(Debug, Clone)]
struct SupportStaff {
    id: Uuid,
    name: PersonName,
    role: SupportStaffRole,
    active: bool,
}

/// Education stream (syllabus)
#[derive(Debug, Clone)]
pub enum Stream {
    CambridgeIGCSE,
    CambridgeLowerSecondary,
    CambridgeALevel,
    WASSCE,
}

/// Subject offered
#[derive(Debug, Clone)]
pub struct Subject {
    pub id: Uuid,
    pub name: SubjectName,
    pub stream: Stream,
}

#[derive(Debug, Clone)]
enum SubjectName {
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

#[derive(Debug, Clone)]
enum AcademicYear {
    Year2024_2025,
    Year2025_2026,
    Year2026_2027,
}

#[derive(Debug, Clone)]
struct Term {
    id: Uuid,
    name: String,
    academic_year: AcademicYear,
    start_date: NaiveDateTime,
    end_date: NaiveDateTime,
    is_active: bool,
}

#[derive(Debug, Clone)]
struct Payment {
    id: Uuid,
    student_id: Uuid,
    term_id: Uuid,
    amount_paid: f64,
    date_paid: NaiveDateTime,
    recorded_by: Uuid,
}

/// Core unit of work done — a taught period
#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
struct ConstraintConfig {
    id: Uuid,
    name: String,         
    base_percentage: f64,
    admin_percentage: f64,
    support_staff_percentage: f64,
    max_periods_paid: Option<u32>,
    max_ratio: Option<f64>,    
    created_at: NaiveDateTime,
}

#[derive(Debug, Clone)]
struct Disbursement {
    id: Uuid,
    term_id: Uuid,
    total_revenue: f64,
    admin_share: f64,
    support_staff_share: f64,
    teacher_base_share: f64,
    teacher_period_share: f64,
    constraint_id: Uuid,
    calculated_on: NaiveDateTime,
}

#[derive(Debug, Clone)]
struct TeacherPayout {
    id: Uuid,
    disbursement_id: Uuid,
    teacher_id: Uuid,
    base_share: f64,
    period_share: f64,
    capped: bool,
}

#[derive(Debug, Clone)]
struct TeacherPayoutItem {
    id: Uuid,
    payout_id: Uuid,
    conducted_period_id: Uuid,
    paid_amount: f64,
}

// =========== UI TYPES =============

/// User information for UI display
#[derive(Debug, Clone)]
pub struct UserProfile {
    pub username: String,
    pub name: PersonName,
    pub role: UserRole,
}

impl From<UserAccount> for UserProfile {
    fn from(account: UserAccount) -> Self {
        Self {
            username: account.username,
            name: account.name,
            role: account.role,
        }
    }
}
