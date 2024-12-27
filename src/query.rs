use crate::query_value::{Query as Q, QueryValue};
use serde::{Deserialize, Serialize};

/// Helper class to generate query strings.
#[derive(Serialize, Deserialize, Debug)]
pub struct Query;

impl Query {
    /// Filter resources where [attribute] is equal to [value].
    ///
    /// [value] can be a single value or a list. If a list is used
    /// the query will return resources where [attribute] is equal
    /// to any of the values in the list.
    /// ```
    /// assert_eq!(
    ///         unofficial_appwrite::query::Query::equal("title".into(), vec!["bamboo", "ace"].into()),
    ///         r#"{"method":"equal","attribute":"title","values":["bamboo","ace"]}"#
    ///     );
    /// ```
    /// or
    /// ```
    /// assert_eq!(
    ///         unofficial_appwrite::query::Query::equal("title".into(), vec!["Iron Man"].into()),
    ///         r#"{"method":"equal","attribute":"title","values":["Iron Man"]}"#
    ///     );
    /// ```
    pub fn equal(attribute: QueryValue, values: QueryValue) -> String {
        Q::new("equal", attribute, values).to_string()
    }

    /// Filter resources where [attribute] is not equal to [value].
    /// ```
    /// assert_eq!(
    ///         unofficial_appwrite::query::Query::not_equal("title".into(), vec!["Iron Man"].into()),
    ///         r#"{"method":"notEqual","attribute":"title","values":["Iron Man"]}"#
    ///     );
    /// ```
    pub fn not_equal(attribute: QueryValue, values: QueryValue) -> String {
        Q::new("notEqual", attribute, values).to_string()
    }

    /// Filter resources where [attribute] is less than [value].
    /// ```
    /// assert_eq!(
    ///         unofficial_appwrite::query::Query::less_than("score".into(), 10.into()),
    ///         r#"{"method":"lessThan","attribute":"score","values":[10]}"#
    ///     );
    /// ```
    pub fn less_than(attribute: QueryValue, value: QueryValue) -> String {
        Q::new("lessThan", attribute, value.convert_to_array_if_not_array()).to_string()
    }

    /// Filter resources where [attribute] is less than or equal to [value].
    /// ```
    /// assert_eq!(
    ///         unofficial_appwrite::query::Query::less_than_equal("score".into(), 10.into()),
    ///         r#"{"method":"lessThanEqual","attribute":"score","values":[10]}"#
    ///     );
    /// ```
    pub fn less_than_equal(attribute: QueryValue, value: QueryValue) -> String {
        Q::new(
            "lessThanEqual",
            attribute,
            value.convert_to_array_if_not_array(),
        )
        .to_string()
    }

    /// Filter resources where [attribute] is greater than [value].
    /// ```
    /// assert_eq!(
    ///         unofficial_appwrite::query::Query::greater_than("score".into(), 10.into()),
    ///         r#"{"method":"greaterThan","attribute":"score","values":[10]}"#
    ///     );
    /// ```
    pub fn greater_than(attribute: QueryValue, value: QueryValue) -> String {
        Q::new(
            "greaterThan",
            attribute,
            value.convert_to_array_if_not_array(),
        )
        .to_string()
    }

    /// Filter resources where [attribute] is greater than or equal to [value].
    /// ```
    /// assert_eq!(
    ///         unofficial_appwrite::query::Query::greater_than_equal("score".into(), 10.into()),
    ///         r#"{"method":"greaterThanEqual","attribute":"score","values":[10]}"#
    ///     );
    /// ```
    pub fn greater_than_equal(attribute: QueryValue, value: QueryValue) -> String {
        Q::new(
            "greaterThanEqual",
            attribute,
            value.convert_to_array_if_not_array(),
        )
        .to_string()
    }

    /// Filter resources where by searching [attribute] for [value].
    /// ```
    /// assert_eq!(
    ///         unofficial_appwrite::query::Query::search("text".into(), "key words".into()),
    ///         r#"{"method":"search","attribute":"text","values":["key words"]}"#
    ///     );
    /// ```
    pub fn search(attribute: QueryValue, value: QueryValue) -> String {
        Q::new("search", attribute, value.convert_to_array_if_not_array()).to_string()
    }

    /// Filter resources where [attribute] is null.
    /// ```
    /// assert_eq!(
    ///         unofficial_appwrite::query::Query::is_null("name".into()),
    ///         r#"{"method":"isNull","attribute":"name"}"#
    ///     );
    /// ```
    pub fn is_null(attribute: QueryValue) -> String {
        Q::new("isNull", attribute, None::<&str>.into()).to_string()
    }

    /// Filter resources where [attribute] is not null.
    /// ```
    /// assert_eq!(
    ///         unofficial_appwrite::query::Query::is_not_null("name".into()),
    ///         r#"{"method":"isNotNull","attribute":"name"}"#
    ///     );
    /// ```
    pub fn is_not_null(attribute: QueryValue) -> String {
        Q::new("isNotNull", attribute, None::<&str>.into()).to_string()
    }

    /// Filter resources where [attribute] is between [start] and [end] (inclusive).
    /// ```
    /// assert_eq!(
    ///         unofficial_appwrite::query::Query::between("price".into(), 5.into(), 10.into()),
    ///         r#"{"method":"between","attribute":"price","values":[5,10]}"#
    ///     );
    /// ```
    pub fn between(attribute: QueryValue, start: QueryValue, end: QueryValue) -> String {
        Q::new("between", attribute, vec![start, end].into()).to_string()
    }

    /// Filter resources where [attribute] starts with [value].
    /// ```
    /// assert_eq!(
    ///         unofficial_appwrite::query::Query::starts_with("name".into(), "Once upon a time".into()),
    ///         r#"{"method":"startsWith","attribute":"name","values":["Once upon a time"]}"#
    ///     );
    /// ```
    pub fn starts_with(attribute: QueryValue, value: QueryValue) -> String {
        Q::new(
            "startsWith",
            attribute,
            value.convert_to_array_if_not_array(),
        )
        .to_string()
    }

    /// Filter resources where [attribute] ends with [value].
    /// ```
    /// assert_eq!(
    ///         unofficial_appwrite::query::Query::ends_with("name".into(), "happily ever after.".into()),
    ///         r#"{"method":"endsWith","attribute":"name","values":["happily ever after."]}"#
    ///     );
    /// ```
    pub fn ends_with(attribute: QueryValue, value: QueryValue) -> String {
        Q::new("endsWith", attribute, value.convert_to_array_if_not_array()).to_string()
    }

    /// Filter resources where [attribute] contains [value]
    /// [value] can be a single value or a list.
    /// ```
    /// assert_eq!(
    ///         unofficial_appwrite::query::Query::contains("ingredients".into(), vec!["apple", "banana"].into()),
    ///         r#"{"method":"contains","attribute":"ingredients","values":["apple","banana"]}"#
    ///     );
    /// ```
    /// or
    /// ```
    /// assert_eq!(
    ///         unofficial_appwrite::query::Query::contains("name".into(), vec!["Tom"].into()),
    ///         r#"{"method":"contains","attribute":"name","values":["Tom"]}"#
    ///     );
    /// ```
    pub fn contains(attribute: QueryValue, value: QueryValue) -> String {
        Q::new("contains", attribute, value).to_string()
    }

    ///```
    /// assert_eq!(
    ///         unofficial_appwrite::query::Query::or(
    ///             vec![
    ///                 unofficial_appwrite::query::Query::less_than("size".into(), 5.into()),
    ///                 unofficial_appwrite::query::Query::greater_than("size".into(), 10.into())
    ///             ]
    ///             .into()
    ///         ),
    ///         r#"{"method":"or","values":[{"attribute":"size","method":"lessThan","values":[5]},{"attribute":"size","method":"greaterThan","values":[10]}]}"#
    ///     );
    /// ```
    pub fn or(queries: Vec<String>) -> String {
        let values = queries
            .into_iter()
            .map(|q| QueryValue::JsonObject(serde_json::from_str(&q).expect("Invalid JSON string")))
            .collect();

        Q::new("or", None::<&str>.into(), QueryValue::Array(values)).to_string()
    }

    ///```
    /// assert_eq!(
    ///         unofficial_appwrite::query::Query::and(vec![
    ///             unofficial_appwrite::query::Query::less_than("size".into(), 10.into()),
    ///             unofficial_appwrite::query::Query::greater_than("size".into(), vec![5].into())
    ///         ]),
    ///         r#"{"method":"and","values":[{"attribute":"size","method":"lessThan","values":[10]},{"attribute":"size","method":"greaterThan","values":[5]}]}"#
    ///     );
    /// ```
    pub fn and(queries: Vec<String>) -> String {
        let values = queries
            .into_iter()
            .map(|q| QueryValue::JsonObject(serde_json::from_str(&q).expect("Invalid JSON string")))
            .collect();
        Q::new("and", None::<&str>.into(), QueryValue::Array(values)).to_string()
    }

    /// Specify which attributes should be returned by the API call.
    pub fn select(attributes: QueryValue) -> String {
        Q::new("select", None::<&str>.into(), attributes).to_string()
    }

    /// Sort results by [attribute] ascending.
    /// ```
    /// assert_eq!(
    ///         unofficial_appwrite::query::Query::order_asc("attribute".into()),
    ///         r#"{"method":"orderAsc","attribute":"attribute"}"#
    ///     );
    /// ```
    pub fn order_asc(attribute: QueryValue) -> String {
        Q::new("orderAsc", attribute, None::<&str>.into()).to_string()
    }

    /// Sort results by [attribute] descending.
    /// ```
    /// assert_eq!(
    ///         unofficial_appwrite::query::Query::order_desc("attribute".into()),
    ///         r#"{"method":"orderDesc","attribute":"attribute"}"#
    ///     );
    /// ```
    pub fn order_desc(attribute: QueryValue) -> String {
        Q::new("orderDesc", attribute, None::<&str>.into()).to_string()
    }

    /// Return results before [id].
    ///
    /// Refer to the [Cursor Based Pagination](https://appwrite.io/docs/pagination#cursor-pagination)
    /// docs for more information.
    /// ```
    /// assert_eq!(
    ///         unofficial_appwrite::query::Query::cursor_before("62a7...a600".into()),
    ///         r#"{"method":"cursorBefore","values":["62a7...a600"]}"#
    ///     );
    /// ```
    pub fn cursor_before(id: QueryValue) -> String {
        Q::new(
            "cursorBefore",
            None::<&str>.into(),
            id.convert_to_array_if_not_array(),
        )
        .to_string()
    }

    /// Return results after [id].
    ///
    /// Refer to the [Cursor Based Pagination](https://appwrite.io/docs/pagination#cursor-pagination)
    /// docs for more information.
    /// ```
    /// assert_eq!(
    ///         unofficial_appwrite::query::Query::cursor_after("62a7...f620".into()),
    ///         r#"{"method":"cursorAfter","values":["62a7...f620"]}"#
    ///     );
    /// ```
    pub fn cursor_after(id: QueryValue) -> String {
        Q::new(
            "cursorAfter",
            None::<&str>.into(),
            id.convert_to_array_if_not_array(),
        )
        .to_string()
    }

    /// Return only [limit] results.
    /// ```
    /// assert_eq!(
    ///         unofficial_appwrite::query::Query::limit(25.into()),
    ///         r#"{"method":"limit","values":[25]}"#
    ///     );
    /// ```
    pub fn limit(limit: QueryValue) -> String {
        Q::new(
            "limit",
            None::<&str>.into(),
            limit.convert_to_array_if_not_array(),
        )
        .to_string()
    }

    /// Return results from [offset].
    ///
    /// Refer to the [Offset Pagination](https://appwrite.io/docs/pagination#offset-pagination)
    /// docs for more information.
    /// ```
    /// assert_eq!(
    ///         unofficial_appwrite::query::Query::offset(0.into()),
    ///         r#"{"method":"offset","values":[0]}"#
    ///     );
    /// ```
    pub fn offset(offset: QueryValue) -> String {
        Q::new(
            "offset",
            None::<&str>.into(),
            offset.convert_to_array_if_not_array(),
        )
        .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_query_at_once() {
        assert_eq!(
            Query::equal("title".into(), vec!["bamboo", "ace"].into()),
            r#"{"method":"equal","attribute":"title","values":["bamboo","ace"]}"#
        );
        assert_eq!(
            Query::and(vec![
                Query::less_than("size".into(), 10.into()),
                Query::greater_than("size".into(), vec![5].into())
            ]),
            r#"{"method":"and","values":[{"attribute":"size","method":"lessThan","values":[10]},{"attribute":"size","method":"greaterThan","values":[5]}]}"#
        );
        assert_eq!(
            Query::or(
                vec![
                    Query::less_than("size".into(), 5.into()),
                    Query::greater_than("size".into(), 10.into())
                ]
                .into()
            ),
            r#"{"method":"or","values":[{"attribute":"size","method":"lessThan","values":[5]},{"attribute":"size","method":"greaterThan","values":[10]}]}"#
        );
        assert_eq!(
            Query::equal("title".into(), vec!["Iron Man"].into()),
            r#"{"method":"equal","attribute":"title","values":["Iron Man"]}"#
        );
        assert_eq!(
            Query::not_equal("title".into(), vec!["Iron Man"].into()),
            r#"{"method":"notEqual","attribute":"title","values":["Iron Man"]}"#
        );
        assert_eq!(
            Query::less_than("score".into(), 10.into()),
            r#"{"method":"lessThan","attribute":"score","values":[10]}"#
        );
        assert_eq!(
            Query::less_than_equal("score".into(), 10.into()),
            r#"{"method":"lessThanEqual","attribute":"score","values":[10]}"#
        );
        assert_eq!(
            Query::greater_than("score".into(), 10.into()),
            r#"{"method":"greaterThan","attribute":"score","values":[10]}"#
        );
        assert_eq!(
            Query::greater_than_equal("score".into(), 10.into()),
            r#"{"method":"greaterThanEqual","attribute":"score","values":[10]}"#
        );
        assert_eq!(
            Query::between("price".into(), 5.into(), 10.into()),
            r#"{"method":"between","attribute":"price","values":[5,10]}"#
        );
        assert_eq!(
            Query::is_null("name".into()),
            r#"{"method":"isNull","attribute":"name"}"#
        );
        assert_eq!(
            Query::is_not_null("name".into()),
            r#"{"method":"isNotNull","attribute":"name"}"#
        );
        assert_eq!(
            Query::starts_with("name".into(), "Once upon a time".into()),
            r#"{"method":"startsWith","attribute":"name","values":["Once upon a time"]}"#
        );
        assert_eq!(
            Query::ends_with("name".into(), "happily ever after.".into()),
            r#"{"method":"endsWith","attribute":"name","values":["happily ever after."]}"#
        );
        assert_eq!(
            Query::contains("ingredients".into(), vec!["apple", "banana"].into()),
            r#"{"method":"contains","attribute":"ingredients","values":["apple","banana"]}"#
        );
        assert_eq!(
            Query::contains("name".into(), vec!["Tom"].into()),
            r#"{"method":"contains","attribute":"name","values":["Tom"]}"#
        );
        assert_eq!(
            Query::search("text".into(), "key words".into()),
            r#"{"method":"search","attribute":"text","values":["key words"]}"#
        );
        assert_eq!(
            Query::order_desc("attribute".into()),
            r#"{"method":"orderDesc","attribute":"attribute"}"#
        );
        assert_eq!(
            Query::order_asc("attribute".into()),
            r#"{"method":"orderAsc","attribute":"attribute"}"#
        );
        assert_eq!(
            Query::limit(25.into()),
            r#"{"method":"limit","values":[25]}"#
        );
        assert_eq!(
            Query::offset(0.into()),
            r#"{"method":"offset","values":[0]}"#
        );
        assert_eq!(
            Query::cursor_after("62a7...f620".into()),
            r#"{"method":"cursorAfter","values":["62a7...f620"]}"#
        );
        assert_eq!(
            Query::cursor_before("62a7...a600".into()),
            r#"{"method":"cursorBefore","values":["62a7...a600"]}"#
        );
    }

    // #[test]
    // fn test_select_query() {
    //     let query = Query::select(vec!["name", "age"]);
    //     assert_eq!(
    //         query.to_string(),
    //         r#"{"method":"select","values":["name","age"]}"#
    //     );
    // }
}
