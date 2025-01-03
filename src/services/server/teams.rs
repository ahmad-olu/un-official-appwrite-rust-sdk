use serde_json::{Map, Value};
///! # Teams
///! The Teams service allows you to group users of your project and to enable
///! them to share read and write access to your project resources
use std::collections::HashMap;

use crate::{
    app_json_header,
    client::Client,
    enumm::HttpMethod,
    error::Error,
    models::{
        membership::Membership, membership_list::MembershipList, preferences::Preferences,
        team::Team, team_list::TeamList,
    },
};

pub struct Teams;

impl Teams {
    /// List teams
    ///
    /// Get a list of all the teams in which the current user is a member. You can
    /// use the parameters to filter your results.
    ///* queries => vec(string)?
    ///* search => string?
    pub async fn list(client: &Client, args: HashMap<String, Value>) -> Result<TeamList, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/teams";

        let api_headers = app_json_header!();

        let res = client
            .call(HttpMethod::GET, api_path, api_headers, &args, None)
            .await?;

        Ok(res.json().await?)
    }

    /// Create team
    ///
    /// Create a new team. The user who creates the team will automatically be
    /// assigned as the owner of the team. Only the users with the owner role can
    /// invite new members, add new owners and delete or update the team.
    ///* teamId => string
    ///* name => string
    ///* roles => vec(string)?
    pub async fn create(client: &Client, args: HashMap<String, Value>) -> Result<Team, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/teams";

        let api_headers = app_json_header!();

        let res = client
            .call(HttpMethod::POST, api_path, api_headers, &args, None)
            .await?;

        Ok(res.json().await?)
    }

    /// Get team
    ///
    /// Get a team by its ID. All team members have read access for this resource.
    pub async fn get(client: &Client, team_id: &str) -> Result<Team, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/teams/{teamId}".replace("{teamId}", team_id);

        let args = HashMap::new();

        let api_headers = app_json_header!();

        let res = client
            .call(HttpMethod::GET, api_path.as_str(), api_headers, &args, None)
            .await?;

        Ok(res.json().await?)
    }

    /// Update name
    ///
    /// Update the team"s name by its unique ID.
    ///* name => string
    pub async fn update_name(
        client: &Client,
        team_id: &str,
        args: HashMap<String, Value>,
    ) -> Result<Team, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/teams/{teamId}".replace("{teamId}", team_id);

        let api_headers = app_json_header!();

        let res = client
            .call(HttpMethod::PUT, api_path.as_str(), api_headers, &args, None)
            .await?;

        Ok(res.json().await?)
    }

    /// Delete team
    ///
    /// Delete a team using its ID. Only team members with the owner role can
    /// delete the team.
    pub async fn delete(client: &Client, team_id: &str) -> Result<(), Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/teams/{teamId}".replace("{teamId}", team_id);

        let args = HashMap::new();

        let api_headers = app_json_header!();

        let _res = client
            .call(
                HttpMethod::DELETE,
                api_path.as_str(),
                api_headers,
                &args,
                None,
            )
            .await?;

        Ok(())
    }

    /// List team memberships
    ///
    /// Use this endpoint to list a team"s members using the team"s ID. All team
    /// members have read access to this endpoint.
    ///* queries => vec(string)?
    ///* search => string?
    pub async fn list_memberships(
        client: &Client,
        team_id: &str,
        args: HashMap<String, Value>,
    ) -> Result<MembershipList, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/teams/{teamId}/memberships".replace("{teamId}", team_id);

        let api_headers = app_json_header!();

        let res = client
            .call(HttpMethod::GET, api_path.as_str(), api_headers, &args, None)
            .await?;

        Ok(res.json().await?)
    }

    /// Create team membership
    ///
    /// Invite a new member to join your team. Provide an ID for existing users, or
    /// invite unregistered users using an email or phone number. If initiated from
    /// a Client SDK, Appwrite will send an email or sms with a link to join the
    /// team to the invited user, and an account will be created for them if one
    /// doesn"t exist. If initiated from a Server SDK, the new member will be added
    /// automatically to the team.
    ///
    /// You only need to provide one of a user ID, email, or phone number. Appwrite
    /// will prioritize accepting the user ID > email > phone number if you provide
    /// more than one of these parameters.
    ///
    /// Use the `url` parameter to redirect the user from the invitation email to
    /// your app. After the user is redirected, use the [Update Team Membership
    /// Status](https://appwrite.io/docs/references/cloud/client-web/teams#updateMembershipStatus)
    /// endpoint to allow the user to accept the invitation to the team.
    ///
    /// Please note that to avoid a [Redirect
    /// Attack](https://github.com/OWASP/CheatSheetSeries/blob/master/cheatsheets/Unvalidated_Redirects_and_Forwards_Cheat_Sheet.md)
    /// Appwrite will accept the only redirect URLs under the domains you have
    /// added as a platform on the Appwrite Console.
    ///
    ///* roles => vec(string)
    ///* email => string?
    ///* userId => string?
    ///* phone => string?
    ///* url => string?
    ///* name => string?
    pub async fn create_memberships(
        client: &Client,
        team_id: &str,
        args: HashMap<String, Value>,
    ) -> Result<Membership, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/teams/{teamId}/memberships".replace("{teamId}", team_id);

        let api_headers = app_json_header!();

        let res = client
            .call(
                HttpMethod::POST,
                api_path.as_str(),
                api_headers,
                &args,
                None,
            )
            .await?;

        Ok(res.json().await?)
    }

    /// Get team membership
    ///
    /// Get a team member by the membership unique id. All team members have read
    /// access for this resource.
    pub async fn get_memberships(
        client: &Client,
        team_id: &str,
        membership_id: &str,
    ) -> Result<Membership, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/teams/{teamId}/memberships/{membershipId}"
            .replace("{teamId}", team_id)
            .replace("{membershipId}", membership_id);

        let args = HashMap::new();

        let api_headers = app_json_header!();

        let res = client
            .call(HttpMethod::GET, api_path.as_str(), api_headers, &args, None)
            .await?;

        Ok(res.json().await?)
    }

    /// Update membership
    ///
    /// Modify the roles of a team member. Only team members with the owner role
    /// have access to this endpoint. Learn more about [roles and
    /// permissions](https://appwrite.io/docs/permissions).
    ///
    ///* userId => vec(string)
    pub async fn update_memberships(
        client: &Client,
        team_id: &str,
        membership_id: &str,
        args: HashMap<String, Value>,
    ) -> Result<Membership, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/teams/{teamId}/memberships/{membershipId}"
            .replace("{teamId}", team_id)
            .replace("{membershipId}", membership_id);

        let api_headers = app_json_header!();

        let res = client
            .call(
                HttpMethod::PATCH,
                api_path.as_str(),
                api_headers,
                &args,
                None,
            )
            .await?;

        Ok(res.json().await?)
    }

    /// Delete team membership
    ///
    /// This endpoint allows a user to leave a team or for a team owner to delete
    /// the membership of any other team member. You can also use this endpoint to
    /// delete a user membership even if it is not accepted.
    pub async fn delete_memberships(
        client: &Client,
        team_id: &str,
        membership_id: &str,
    ) -> Result<(), Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/teams/{teamId}/memberships/{membershipId}"
            .replace("{teamId}", team_id)
            .replace("{membershipId}", membership_id);

        let args = HashMap::new();

        let api_headers = app_json_header!();

        let _res = client
            .call(
                HttpMethod::DELETE,
                api_path.as_str(),
                api_headers,
                &args,
                None,
            )
            .await?;

        Ok(())
    }

    /// Update team membership status
    ///
    /// Use this endpoint to allow a user to accept an invitation to join a team
    /// after being redirected back to your app from the invitation email received
    /// by the user.
    ///
    /// If the request is successful, a session for the user is automatically
    /// created.
    ///
    ///* userId => string
    ///* secret => string
    pub async fn update_membership_status(
        client: &Client,
        team_id: &str,
        membership_id: &str,
        args: HashMap<String, Value>,
    ) -> Result<Membership, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/teams/{teamId}/memberships/{membershipId}/status"
            .replace("{teamId}", team_id)
            .replace("{membershipId}", membership_id);

        let api_headers = app_json_header!();

        let res = client
            .call(
                HttpMethod::PATCH,
                api_path.as_str(),
                api_headers,
                &args,
                None,
            )
            .await?;

        Ok(res.json().await?)
    }

    /// Get team preferences
    ///
    /// Get the team's shared preferences by its unique ID. If a preference doesn't
    /// need to be shared by all team members, prefer storing them in [user
    /// preferences](https://appwrite.io/docs/references/cloud/client-web/account#getPrefs).
    pub async fn get_prefs(client: &Client, team_id: &str) -> Result<Preferences, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/teams/{teamId}/prefs".replace("{teamId}", team_id);

        let args = HashMap::new();

        let api_headers = app_json_header!();

        let res = client
            .call(HttpMethod::GET, api_path.as_str(), api_headers, &args, None)
            .await?;

        Ok(res.json().await?)
    }

    /// Update preferences
    ///
    /// Update the team's preferences by its unique ID. The object you pass is
    /// stored as is and replaces any previous value. The maximum allowed prefs
    /// size is 64kB and throws an error if exceeded.
    ///* prefs => HashMap<String, Value>
    pub async fn update_prefs(
        client: &Client,
        team_id: &str,
        args: HashMap<String, Value>,
    ) -> Result<Preferences, Error> {
        //const API_PATH: &str = "/functions";
        let api_path = "/teams/{teamId}/prefs".replace("{teamId}", team_id);

        let api_headers = app_json_header!();

        let res = client
            .call(HttpMethod::PUT, api_path.as_str(), api_headers, &args, None)
            .await?;

        Ok(res.json().await?)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use serde_json::{json, Map, Value};

    use crate::{
        client::ClientBuilder, error::Error, id::ID, query::Query, role::Role,
        services::server::users::Users,
    };

    use super::Teams;

    // #[tokio::test]
    async fn test_teams() -> Result<(), Error> {
        let client = ClientBuilder::default()
            .set_endpoint("http://127.0.0.1/v1")?
            .set_project("676c2b7b000c834e1fce")?
            .set_key("standard_5d84014ebaf0de52308eff28946a43062921240c10b81c2fd037ab60b02f0257b7f0a53fe94065170fe7c7d0af2d4136d4cbf32a4055baeada3d27f2e323b70aeda87e97f676207cf10cbb18b7a80f8d1103803617454c89138f217dad701bbe9dc6950bc58853fdb2a0b4b67d2a8b8b6b7b9b2e6d9b94e0a2fcfee794688e2e")?
            //.set_self_signed(false)?
            .build()?;

        // ! create user
        let create_user1 = Users::create(
            &client,
            maplit::hashmap! {
                "userId".into() => ID::unique(7).into(),
                "email".into()=> "fakeEmailTeams1@Email.com".into(),
                "password".into()=> "VeryVerySecurePassword@123456789".into(),
                "name".into()=> "fakeEmail11".into()
            },
        )
        .await?;
        assert_eq!(create_user1.email.clone(), "fakeemailteams1@email.com");

        let create_user2 = Users::create(
            &client,
            maplit::hashmap! {
                "userId".into() => ID::unique(7).into(),
                "email".into()=> "fakeEmailTeams2@Email.com".into(),
                "password".into()=> "VeryVerySecurePassword@123456789".into(),
                "name".into()=> "fakeEmail22".into()
            },
        )
        .await?;
        assert_eq!(create_user2.email, "fakeemailteams2@email.com");

        // ! create team
        let create_team = Teams::create(
            &client,
            maplit::hashmap! {
                "teamId".into() => ID::unique(7).into(),
                "name".into()=> "boston org".into(),
                "roles".into()=> vec!["maths".to_string(), "geography".to_string(),"english".to_string()].into()
            },
        )
        .await?;
        assert_eq!(create_team.clone().name, "boston org");

        // ! create members
        let users = [create_user1.clone(), create_user2.clone()];
        let mut membership_ids: [String; 2] = [String::new(), String::new()];
        for user in users {
            let roles: Vec<String> = if create_user1.email == "fakeemailteams1@email.com" {
                vec!["maths".to_string(), "geography".to_string()]
            } else {
                vec!["maths".to_string()]
            };
            let create_members = Teams::create_memberships(
                &client,
                &create_team.id,
                maplit::hashmap! {
                    "roles".into()=> roles.into(),
                    "userId".into() => user.id.clone().into(),
                    "email".into()=> user.email.clone().into(),
                    "name".into()=> user.name.clone().into(),
                },
            )
            .await?;
            if create_user1.email == "fakeemailteams1@email.com" {
                membership_ids[0] = create_members.id;
            } else {
                membership_ids[1] = create_members.id;
            };
            assert_eq!(create_members.user_email, user.email);
            assert_eq!(create_members.user_name, user.name);
            assert_eq!(create_members.user_id, user.id);
        }

        // ! update team preference
        let prefs_val: HashMap<String, Value> = maplit::hashmap! {
            "team_length".into()=> 15.into(),
            "team_location".into()=> "cairo".into(),
        };

        let update_team_pref = Teams::update_prefs(
            &client,
            &create_team.id,
            maplit::hashmap! {
                "prefs".into() => Value::Object(prefs_val.clone().into_iter().collect::<Map<String, Value>>()),
            },
        )
        .await?;
        assert_eq!(update_team_pref.data, prefs_val);

        // ! update team
        let update_team = Teams::update_name(
            &client,
            &create_team.id,
            maplit::hashmap! {
                "name".into()=> "chicago org".into(),
            },
        )
        .await?;
        assert_eq!(update_team.clone().name, "chicago org");

        // ! remove members
        let remove_member =
            Teams::delete_memberships(&client, &create_team.id, &membership_ids[0]).await?;
        assert_eq!(remove_member, ());

        // ! delete team and members
        let delete_team = Teams::delete(&client, &create_team.id).await?;
        assert_eq!(delete_team, ());
        let delete_user1 = Users::delete(&client, &create_user1.id).await?;
        assert_eq!(delete_user1, ());
        let delete_user2 = Users::delete(&client, &create_user2.id).await?;
        assert_eq!(delete_user2, ());

        Ok(())
    }
}
