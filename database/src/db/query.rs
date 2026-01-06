//! # query
//!
//! query 模块
//!
//! @author TimonQWQ
//! @date 2026-01-06

//! Query builder

//! // Join type for SQL JOIN operations
#[derive(Debug, Clone)]
pub enum JoinType {
    Inner,
    Left,
    Right,
    FullOuter,
}

/// Join clause information
#[derive(Debug, Clone)]
pub struct JoinClause {
    pub join_type: JoinType,
    pub table: String,
    pub condition: String,
}

/// Where condition operator
#[derive(Debug, Clone, Copy)]
pub enum WhereOperator {
    And,
    Or,
}

/// Where condition group
#[derive(Debug, Clone)]
pub struct WhereGroup {
    pub operator: WhereOperator,
    pub conditions: Vec<String>,
}

/// Parameter binding for safe SQL queries
#[derive(Debug, Clone)]
pub struct QueryParam {
    pub name: String,
    pub value: String, // Serialized value
}

/// Type-safe parameter value
#[derive(Debug, Clone)]
pub enum ParamValue {
    String(String),
    Int(i64),
    Float(f64),
    Bool(bool),
    Null,
}

impl ParamValue {
    /// Convert to SQL string representation
    pub fn to_sql_string(&self) -> String {
        match self {
            ParamValue::String(s) => format!("'{}'", s.replace("'", "''")),
            ParamValue::Int(i) => i.to_string(),
            ParamValue::Float(f) => f.to_string(),
            ParamValue::Bool(b) => if *b { "TRUE".to_string() } else { "FALSE".to_string() },
            ParamValue::Null => "NULL".to_string(),
        }
    }
    
    /// Serialize to JSON string for storage
    pub fn to_json_string(&self) -> String {
        match self {
            ParamValue::String(s) => format!("\"{}\"", s.replace("\\", "\\\\").replace("\"", "\\\"")),
            ParamValue::Int(i) => i.to_string(),
            ParamValue::Float(f) => f.to_string(),
            ParamValue::Bool(b) => b.to_string(),
            ParamValue::Null => "null".to_string(),
        }
    }
}

/// Parameterized query builder
pub struct ParameterizedQuery {
    pub sql: String,
    pub params: Vec<QueryParam>,
}

/// Query builder for constructing SQL queries
pub struct QueryBuilder {
    where_clauses: Vec<String>,
    where_groups: Vec<WhereGroup>,
    order_by: Option<(String, String)>,
    group_by: Option<String>,
    having: Option<String>,
    limit_value: Option<usize>,
    offset_value: Option<usize>,
    joins: Vec<JoinClause>,
    subqueries: Vec<(String, String)>, // (alias, subquery)
    params: Vec<QueryParam>, // Parameter bindings
}

impl QueryBuilder {
    /// Create a new query builder
    pub fn new() -> Self {
        Self {
            where_clauses: Vec::new(),
            where_groups: Vec::new(),
            order_by: None,
            group_by: None,
            having: None,
            limit_value: None,
            offset_value: None,
            joins: Vec::new(),
            subqueries: Vec::new(),
            params: Vec::new(),
        }
    }

    /// Add WHERE condition
    pub fn where_condition(mut self, condition: &str, _args: Vec<&dyn sqlx::Encode<'_, sqlx::Postgres>>) -> Self {
        self.where_clauses.push(condition.to_string());
        self
    }

    /// Add WHERE condition (simplified)
    pub fn where_simple(mut self, condition: &str) -> Self {
        self.where_clauses.push(condition.to_string());
        self
    }

    /// Add WHERE condition with parameter binding (safe)
    pub fn where_param(mut self, field: &str, param_name: &str, value: &str) -> Self {
        let condition = format!("{} = :{}", field, param_name);
        self.where_clauses.push(condition);
        self.params.push(QueryParam {
            name: param_name.to_string(),
            value: value.to_string(),
        });
        self
    }

    /// Add WHERE condition with type-safe parameter binding
    pub fn where_eq<T: Into<ParamValue>>(mut self, field: &str, value: T) -> Self {
        let param_name = format!("param_{}", self.params.len());
        let param_value = value.into();
        let condition = format!("{} = :{}", field, param_name);
        self.where_clauses.push(condition);
        self.params.push(QueryParam {
            name: param_name.clone(),
            value: param_value.to_json_string(),
        });
        self
    }

    /// Add WHERE NOT EQUAL condition
    pub fn where_ne<T: Into<ParamValue>>(mut self, field: &str, value: T) -> Self {
        let param_name = format!("param_{}", self.params.len());
        let param_value = value.into();
        let condition = format!("{} != :{}", field, param_name);
        self.where_clauses.push(condition);
        self.params.push(QueryParam {
            name: param_name.clone(),
            value: param_value.to_json_string(),
        });
        self
    }

    /// Add WHERE LIKE condition
    pub fn where_like(mut self, field: &str, pattern: &str) -> Self {
        let param_name = format!("param_{}", self.params.len());
        let condition = format!("{} LIKE :{}", field, param_name);
        self.where_clauses.push(condition);
        self.params.push(QueryParam {
            name: param_name.clone(),
            value: pattern.to_string(),
        });
        self
    }

    /// Add WHERE IN condition with type-safe parameter binding
    pub fn where_in<T: Into<ParamValue> + Clone>(mut self, field: &str, values: &[T]) -> Self {
        if values.is_empty() {
            // Empty IN clause - always false
            self.where_clauses.push("1 = 0".to_string());
            return self;
        }
        
        let placeholders: Vec<String> = (0..values.len())
            .map(|i| format!(":param_{}", self.params.len() + i))
            .collect();
        let condition = format!("{} IN ({})", field, placeholders.join(", "));
        self.where_clauses.push(condition);
        for value in values {
            let param_value: ParamValue = value.clone().into();
            self.params.push(QueryParam {
                name: format!("param_{}", self.params.len()),
                value: param_value.to_json_string(),
            });
        }
        self
    }

    /// Add WHERE NOT IN condition
    pub fn where_not_in<T: Into<ParamValue> + Clone>(mut self, field: &str, values: &[T]) -> Self {
        if values.is_empty() {
            // Empty NOT IN clause - always true (no condition needed)
            return self;
        }
        
        let placeholders: Vec<String> = (0..values.len())
            .map(|i| format!(":param_{}", self.params.len() + i))
            .collect();
        let condition = format!("{} NOT IN ({})", field, placeholders.join(", "));
        self.where_clauses.push(condition);
        for value in values {
            let param_value: ParamValue = value.clone().into();
            self.params.push(QueryParam {
                name: format!("param_{}", self.params.len()),
                value: param_value.to_json_string(),
            });
        }
        self
    }

    /// Add WHERE condition with parameter binding (legacy method for backward compatibility)
    pub fn where_in_legacy(mut self, field: &str, param_name: &str, values: Vec<String>) -> Self {
        if values.is_empty() {
            self.where_clauses.push("1 = 0".to_string());
            return self;
        }
        
        let placeholders: Vec<String> = (0..values.len())
            .map(|i| format!(":{}_{}", param_name, i))
            .collect();
        let condition = format!("{} IN ({})", field, placeholders.join(", "));
        self.where_clauses.push(condition);
        for (i, value) in values.iter().enumerate() {
            self.params.push(QueryParam {
                name: format!("{}_{}", param_name, i),
                value: value.clone(),
            });
        }
        self
    }

    /// Add WHERE BETWEEN condition
    pub fn where_between<T: Into<ParamValue>>(mut self, field: &str, min: T, max: T) -> Self {
        let param_min = format!("param_{}", self.params.len());
        let param_max = format!("param_{}", self.params.len() + 1);
        let condition = format!("{} BETWEEN :{} AND :{}", field, param_min, param_max);
        self.where_clauses.push(condition);
        let min_value: ParamValue = min.into();
        let max_value: ParamValue = max.into();
        self.params.push(QueryParam {
            name: param_min.clone(),
            value: min_value.to_json_string(),
        });
        self.params.push(QueryParam {
            name: param_max.clone(),
            value: max_value.to_json_string(),
        });
        self
    }

    /// Add WHERE IS NULL condition
    pub fn where_null(mut self, field: &str) -> Self {
        let condition = format!("{} IS NULL", field);
        self.where_clauses.push(condition);
        self
    }

    /// Add WHERE IS NOT NULL condition
    pub fn where_not_null(mut self, field: &str) -> Self {
        let condition = format!("{} IS NOT NULL", field);
        self.where_clauses.push(condition);
        self
    }

    /// Add WHERE condition with comparison operator
    pub fn where_cmp<T: Into<ParamValue>>(mut self, field: &str, op: &str, value: T) -> Self {
        let param_name = format!("param_{}", self.params.len());
        let param_value = value.into();
        let condition = format!("{} {} :{}", field, op, param_name);
        self.where_clauses.push(condition);
        self.params.push(QueryParam {
            name: param_name.clone(),
            value: param_value.to_json_string(),
        });
        self
    }

    /// Validate query builder (basic validation)
    pub fn validate(&self) -> Result<(), String> {
        // Check for empty WHERE clause with OR operator
        if !self.where_groups.is_empty() && self.where_clauses.is_empty() {
            return Err("Cannot use OR operator without base WHERE conditions".to_string());
        }
        
        // Check for valid LIMIT/OFFSET
        if let Some(offset) = self.offset_value {
            if offset == 0 && self.limit_value.is_none() {
                return Err("OFFSET without LIMIT may cause performance issues".to_string());
            }
        }
        
        Ok(())
    }

    /// Get parameterized query
    pub fn to_parameterized(self, base_sql: &str) -> ParameterizedQuery {
        let sql = self.build_select(base_sql);
        ParameterizedQuery {
            sql,
            params: self.params,
        }
    }

    /// Get parameters
    pub fn params(&self) -> &[QueryParam] {
        &self.params
    }

    /// Add WHERE condition with AND operator
    pub fn and_where(mut self, condition: &str) -> Self {
        self.where_clauses.push(condition.to_string());
        self
    }

    /// Add WHERE condition with OR operator
    pub fn or_where(mut self, condition: &str) -> Self {
        if !self.where_clauses.is_empty() {
            // Create a new group for OR conditions
            let mut group = WhereGroup {
                operator: WhereOperator::Or,
                conditions: Vec::new(),
            };
            // Move last condition to group
            if let Some(last) = self.where_clauses.pop() {
                group.conditions.push(last);
            }
            group.conditions.push(condition.to_string());
            self.where_groups.push(group);
        } else {
            self.where_clauses.push(condition.to_string());
        }
        self
    }

    /// Add a WHERE group (for complex conditions)
    pub fn where_group<F>(mut self, operator: WhereOperator, builder: F) -> Self
    where
        F: FnOnce(QueryBuilder) -> QueryBuilder,
    {
        let mut sub_builder = QueryBuilder::new();
        sub_builder = builder(sub_builder);
        let conditions = sub_builder.where_clauses;
        if !conditions.is_empty() {
            self.where_groups.push(WhereGroup {
                operator,
                conditions,
            });
        }
        self
    }

    /// Add a subquery
    pub fn subquery(mut self, alias: &str, subquery: &str) -> Self {
        self.subqueries.push((alias.to_string(), subquery.to_string()));
        self
    }

    /// Add ORDER BY clause
    pub fn order_by(mut self, field: &str, order: &str) -> Self {
        self.order_by = Some((field.to_string(), order.to_string()));
        self
    }

    /// Add GROUP BY clause
    pub fn group_by(mut self, field: &str) -> Self {
        self.group_by = Some(field.to_string());
        self
    }

    /// Add HAVING clause
    pub fn having(mut self, condition: &str) -> Self {
        self.having = Some(condition.to_string());
        self
    }

    /// Set LIMIT
    pub fn limit(mut self, limit: usize) -> Self {
        self.limit_value = Some(limit);
        self
    }

    /// Set OFFSET
    pub fn offset(mut self, offset: usize) -> Self {
        self.offset_value = Some(offset);
        self
    }

    /// Add JOIN clause
    pub fn join(mut self, join_type: JoinType, table: &str, condition: &str) -> Self {
        self.joins.push(JoinClause {
            join_type,
            table: table.to_string(),
            condition: condition.to_string(),
        });
        self
    }

    /// Add INNER JOIN
    pub fn inner_join(mut self, table: &str, condition: &str) -> Self {
        self.joins.push(JoinClause {
            join_type: JoinType::Inner,
            table: table.to_string(),
            condition: condition.to_string(),
        });
        self
    }

    /// Add LEFT JOIN
    pub fn left_join(mut self, table: &str, condition: &str) -> Self {
        self.joins.push(JoinClause {
            join_type: JoinType::Left,
            table: table.to_string(),
            condition: condition.to_string(),
        });
        self
    }

    /// Add RIGHT JOIN
    pub fn right_join(mut self, table: &str, condition: &str) -> Self {
        self.joins.push(JoinClause {
            join_type: JoinType::Right,
            table: table.to_string(),
            condition: condition.to_string(),
        });
        self
    }

    /// Add FULL OUTER JOIN
    pub fn full_outer_join(mut self, table: &str, condition: &str) -> Self {
        self.joins.push(JoinClause {
            join_type: JoinType::FullOuter,
            table: table.to_string(),
            condition: condition.to_string(),
        });
        self
    }

    /// Build SELECT query
    pub fn build_select(&self, base_sql: &str) -> String {
        let mut sql = base_sql.to_string();
        
        // Add subqueries as FROM clauses if any
        if !self.subqueries.is_empty() {
            for (alias, subquery) in &self.subqueries {
                sql.push_str(&format!(" FROM ({}) AS {}", subquery, alias));
            }
        }
        
        // Add JOIN clauses
        for join in &self.joins {
            let join_str = match join.join_type {
                JoinType::Inner => "INNER JOIN",
                JoinType::Left => "LEFT JOIN",
                JoinType::Right => "RIGHT JOIN",
                JoinType::FullOuter => "FULL OUTER JOIN",
            };
            sql.push_str(&format!(" {} {} ON {}", join_str, join.table, join.condition));
        }
        
        // Build WHERE clause with groups
        let mut where_parts = Vec::new();
        if !self.where_clauses.is_empty() {
            where_parts.push(self.where_clauses.join(" AND "));
        }
        for group in &self.where_groups {
            let operator = match group.operator {
                WhereOperator::And => "AND",
                WhereOperator::Or => "OR",
            };
            let group_sql = format!("({})", group.conditions.join(&format!(" {} ", operator)));
            where_parts.push(group_sql);
        }
        if !where_parts.is_empty() {
            sql.push_str(" WHERE ");
            sql.push_str(&where_parts.join(" AND "));
        }
        
        if let Some(ref group_by) = self.group_by {
            sql.push_str(&format!(" GROUP BY {}", group_by));
        }
        
        if let Some(ref having) = self.having {
            sql.push_str(&format!(" HAVING {}", having));
        }
        
        if let Some((ref field, ref order)) = self.order_by {
            sql.push_str(&format!(" ORDER BY {} {}", field, order));
        }
        
        if let Some(limit) = self.limit_value {
            sql.push_str(&format!(" LIMIT {}", limit));
        }
        
        if let Some(offset) = self.offset_value {
            sql.push_str(&format!(" OFFSET {}", offset));
        }
        
        sql
    }

    /// Build UPDATE query
    pub fn build_update(&self, base_sql: &str) -> String {
        let mut sql = base_sql.to_string();
        
        // Build WHERE clause with groups
        let mut where_parts = Vec::new();
        if !self.where_clauses.is_empty() {
            where_parts.push(self.where_clauses.join(" AND "));
        }
        for group in &self.where_groups {
            let operator = match group.operator {
                WhereOperator::And => "AND",
                WhereOperator::Or => "OR",
            };
            let group_sql = format!("({})", group.conditions.join(&format!(" {} ", operator)));
            where_parts.push(group_sql);
        }
        if !where_parts.is_empty() {
            sql.push_str(" WHERE ");
            sql.push_str(&where_parts.join(" AND "));
        }
        
        sql
    }

    /// Build DELETE query
    pub fn build_delete(&self, base_sql: &str) -> String {
        let mut sql = base_sql.to_string();
        
        // Build WHERE clause with groups
        let mut where_parts = Vec::new();
        if !self.where_clauses.is_empty() {
            where_parts.push(self.where_clauses.join(" AND "));
        }
        for group in &self.where_groups {
            let operator = match group.operator {
                WhereOperator::And => "AND",
                WhereOperator::Or => "OR",
            };
            let group_sql = format!("({})", group.conditions.join(&format!(" {} ", operator)));
            where_parts.push(group_sql);
        }
        if !where_parts.is_empty() {
            sql.push_str(" WHERE ");
            sql.push_str(&where_parts.join(" AND "));
        }
        
        sql
    }
}

impl Default for QueryBuilder {
    fn default() -> Self {
        Self::new()
    }
}

// Implement Into<ParamValue> for common types
impl From<String> for ParamValue {
    fn from(s: String) -> Self {
        ParamValue::String(s)
    }
}

impl From<&str> for ParamValue {
    fn from(s: &str) -> Self {
        ParamValue::String(s.to_string())
    }
}

impl From<i32> for ParamValue {
    fn from(i: i32) -> Self {
        ParamValue::Int(i as i64)
    }
}

impl From<i64> for ParamValue {
    fn from(i: i64) -> Self {
        ParamValue::Int(i)
    }
}

impl From<f32> for ParamValue {
    fn from(f: f32) -> Self {
        ParamValue::Float(f as f64)
    }
}

impl From<f64> for ParamValue {
    fn from(f: f64) -> Self {
        ParamValue::Float(f)
    }
}

impl From<bool> for ParamValue {
    fn from(b: bool) -> Self {
        ParamValue::Bool(b)
    }
}

