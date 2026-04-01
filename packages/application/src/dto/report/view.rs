use domain::{
    report::report_type::ReportTypeId,
    value_objects::{Body, Comment, DateTime, Url},
    Description, Name, ReportId, ReportStatus, TenantId, Title, UserId,
};

use std::collections::HashSet;

#[derive(Debug, Default)]
pub struct ReportView {
    pub id: Option<ReportId>,
    pub title: Option<Title>,
    pub content: Option<ReportContentView>,
    pub report_type: Option<ReportTypeView>,
    pub status: Option<ReportStatus>,
    pub author_id: Option<UserId>,
    pub owner_tenant: Option<TenantId>,
    pub shared_with_tenants: HashSet<TenantId>,
    pub assigned_reviewer_id: HashSet<UserId>,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub due_date: Option<DateTime>,
    pub version: Option<u64>,
}

#[derive(Debug, Default)]
pub struct ReviewCommentView {
    pub reviewer_id: Option<UserId>,
    pub comment: Option<Comment>,
    pub created_at: Option<DateTime>,
}

#[derive(Debug)]
pub struct ReportContentView {
    pub body: Option<Body>,
    pub attachments: Option<HashSet<Url>>, // URLs or paths to attachments
    pub review_comments: Option<HashSet<ReviewCommentView>>,
    pub rejection_reason: Option<Option<Comment>>,
}

#[derive(Debug)]
pub struct ReportTypeView {
    pub id: Option<ReportTypeId>,
    pub name: Option<Name>,
    pub description: Option<Description>,
    pub created_at: Option<DateTime>,
}
