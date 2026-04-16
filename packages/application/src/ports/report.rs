use domain::{value_objects::Title, Report, ReportId, UserId};

use crate::{SubjectContex, dto::report::{command::ReportCommand, view::ReportView}, error::AppResult};

use super::SortBy;


#[derive(Debug, Clone)]
pub enum ReportQueryResult {
    Single(ReportView),
    Array(Vec<ReportView>),
    None,
}

impl ReportQueryResult {
    pub fn get_array(&self) -> Option<Vec<ReportView>> {
        match self {
            ReportQueryResult::Array(reports) => Some(reports.to_vec()),
            _ => None,
        }
    }
    pub fn get_value(&self) -> Option<ReportView> {
        match self {
            ReportQueryResult::Single(report) => Some(report.clone()),
            _ => None,
        }
    }
}



#[async_trait::async_trait]
pub trait ReportRepository {
    async fn create(&self,ctx: SubjectContex, report: ReportCommand) -> AppResult<ReportView>;
    async fn delete(&self,ctx: SubjectContex, report_id: ReportId) -> AppResult<bool>;
    async fn update(&self,ctx: SubjectContex, report: ReportCommand) -> AppResult<ReportView>;
    async fn get_by_id(&self,ctx: SubjectContex, id: ReportId) -> AppResult<ReportView>;
    async fn get_by_author_id(&self,ctx: SubjectContex,sort_by: &[SortBy], page: u32, page_size: u32, auther_id: UserId) -> AppResult<Vec<ReportView>>;
    async fn get_by_title(&self,ctx: SubjectContex, title: Title) -> AppResult<ReportView>;
    async fn get_reports_paginated(&self,ctx: SubjectContex,sort_by: &[SortBy], page: u32, page_size: u32) -> AppResult<Vec<ReportView>>;
    async fn raw_query(&self,ctx: SubjectContex, query: String) -> AppResult<ReportQueryResult>;
}
