use async_trait::async_trait;
use application::{RequestContex, error::{AppResult, ApplicationError}, ports::{ ReportRepository, SortBy, report::ReportQueryResult}};
use domain::{Title,user::UserId, report::{Report, ReportId}};

use crate::{
    database::client::SurrealDBClient, error::InfrastructureError,
    serialization::{
        SerializedReportId,SerializedUserId, report::{SurrealReportResponseExt, report::SerializedReport}, value_objects::{SerializedName, SerializedTitle},
        
    }
};

pub struct SurrealReportRepository {
    client: SurrealDBClient,
}

impl SurrealReportRepository {
    pub fn new(client: SurrealDBClient) -> Self {
        Self { client }
    }
}

// TODO: Permission must have logic for ranking it
#[async_trait]
impl ReportRepository for SurrealReportRepository {
    async fn create(&self,ctx: RequestContex, report: Report) -> AppResult<Report>{
        let record: SerializedReport = report.try_into().map_err(|_| ApplicationError::Repository("Report Error ".to_string()))?;
        let result: Option<SerializedReport> =  self
            .client
            .db
            .query("LET $report_id = $uid;
                    INSERT INTO report CONTENT $report RETURN AFTER")
            .bind(("uid", ctx.user_id_as_str()))
            .bind(("report", record))
            .await.map_err(|err| ApplicationError::Repository(err.to_string()))?.take(0)  // Get first query result
            .map_err(|err| ApplicationError::Repository(err.to_string()))?;
        match result {
            Some(report) => Ok(report.try_into()?),
            None => Err(ApplicationError::Repository("Report not created!".to_string())),
        }
        
    }
    async fn delete(&self,ctx: RequestContex, id: ReportId) -> AppResult<bool>{
        let result: Option<SerializedReport> =  self
            .client
            .db
            .query("LET $report_id = $uid;
                    DELETE person:$id RETURN BEFORE")
            .bind(("uid", ctx.user_id_as_str()))
            .bind(("id", id.id()))
            .await.map_err(|err| ApplicationError::Repository(err.to_string()))?.take(0)  // Get first query result
            .map_err(|err| ApplicationError::Repository(err.to_string()))?;
        match result {
            Some(_) => Ok(true),
            None => Err(ApplicationError::Repository("Report not deleted!".to_string())),
        }
    }
    async fn update(&self,ctx: RequestContex, report: Report) -> AppResult<Report>{
        let record: SerializedReport = report.try_into().map_err(|err: InfrastructureError| ApplicationError::Domain(err.into()))?;
        let result: Option<SerializedReport> =  self
            .client
            .db
            .query("LET $report_id = $uid;
                    UPDATE report:$id MERGE $report RETURN AFTER")
            .bind(("report", record.clone()))
            .bind(("uid", ctx.user_id_as_str()))
            .bind(("id", record.id()))
            .await.map_err(|err| ApplicationError::Repository(err.to_string()))?.take(0)  // Get first query result
            .map_err(|err| ApplicationError::Repository(err.to_string()))?;
        match result {
            Some(report) => Ok(report.try_into()?),
            None => Err(ApplicationError::Repository("Report not Updated!".to_string())),
        }
    }
    async fn get_by_id(&self, _request_contex: RequestContex, id: ReportId) -> AppResult<Report>{
        let id: SerializedReportId = id.into();
        let result: Option<SerializedReport> =  self
            .client
            .db
            .query("SELECT * FROM ONLY report:$id")
            .bind(("id", id))
            .await.map_err(|err| ApplicationError::Repository(err.to_string()))?.take(0)  // Get first query result
            .map_err(|err| ApplicationError::Repository(err.to_string()))?;
        match result {
            Some(report) => Ok(report.try_into()?),
            None => Err(ApplicationError::Repository("Report not Updated!".to_string())),
        }
    }

    async fn get_by_author_id(&self,ctx: RequestContex,sort_by: &[SortBy], page: u32, page_size: u32, auther_id: UserId) -> AppResult<Vec<Report>>{
        let auther_id: SerializedUserId = auther_id.into();
        let mut order = String::new();
        
        for ord in sort_by{
            order.push_str(&format!("{},", ord.as_string()));
        }
        if !sort_by.is_empty(){
            order = format!("ORDER BY {}", order);
            order.truncate(order.len() -1);
        }
        
        let result: Vec<SerializedReport> =  self
            .client
            .db
            .query("SELECT * FROM report WHERE auther_id = $auther_id  $order LIMIT $page_size START $start_at")
            .bind(("auther_id", auther_id))
            .bind(("order", order))
            .bind(("page_size", page_size))
            .bind(("start_at", page*page_size))
            .await.map_err(|err| ApplicationError::Repository(err.to_string()))?.take(0).map_err(|err| ApplicationError::Repository(err.to_string()))?;
        let mut reports: Vec<Report> = Vec::new();
        for report in result{
            reports.push(report.try_into().map_err(|err: InfrastructureError| ApplicationError::Repository(err.to_string()))?);
        }
        Ok(reports)
    }
    
    async fn get_by_title(&self, _request_contex:RequestContex, title: Title) -> AppResult<Report>{
        let title: SerializedTitle = title.try_into()?;
        let result: Option<SerializedReport> =  self
            .client
            .db
            .query("SELECT * FROM report WHERE title = $title LIMIT 1")
            .bind(("title", title))
            .await.map_err(|err| ApplicationError::Repository(err.to_string()))?.take(0)  // Get first query result
            .map_err(|err| ApplicationError::Repository(err.to_string()))?;
        match result {
            Some(report) => Ok(report.try_into()?),
            None => Err(ApplicationError::Repository("Report not Updated!".to_string())),
        }
    }
    // TODO: Add permissions field into requert contex to be prossesed as required take into account murge all reports permessions with the report ones
// TODO: Add permissions field into requert contex to be prossesed as required take into account murge all reports permessions with the user ones
    async fn get_reports_paginated(&self,ctx: RequestContex ,sort_by: &[SortBy], page: u32, page_size: u32) -> AppResult<Vec<Report>>{
        let mut order = String::new();
        
        for ord in sort_by{
            order.push_str(&format!("{},", ord.as_string()));
        }
        if !sort_by.is_empty(){
            order = format!("ORDER BY {}", order);
            order.truncate(order.len() -1);
        }
        
        let result: Vec<SerializedReport> =  self
            .client
            .db
            .query("SELECT * FROM report $order LIMIT $page_size START $start_at")
            .bind(("order", order))
            .bind(("page_size", page_size))
            .bind(("start_at", page*page_size))
            .await.map_err(|err| ApplicationError::Repository(err.to_string()))?.take(0).map_err(|err| ApplicationError::Repository(err.to_string()))?;
        let mut reports: Vec<Report> = Vec::new();
        for report in result{
            reports.push(report.try_into().map_err(|err: InfrastructureError| ApplicationError::Repository(err.to_string()))?);
        }
        Ok(reports)
    }
    async fn raw_query(&self,ctx: RequestContex, query: String) -> AppResult<ReportQueryResult>{
        let response = self.client
            .db
            .query(query)
            .await.map_err(|err| ApplicationError::Repository(err.to_string()))?;
        // Using the extension trait
        Ok(response.into_report_result().await.map_err(|err| ApplicationError::Repository(err.to_string()))?.try_into()?)
    }
}



