use chrono::offset::Utc;
use mongodb::{bson::doc, results::DeleteResult, Client, Collection};
use std::collections::HashMap;
use uuid::Uuid;

use crate::{
    error::Error,
    models::user::{NewUser, User},
};

#[derive(Clone)]
pub struct MongodbBackend {
    client: Client,
    config: HashMap<String, String>,
}

impl MongodbBackend {
    pub async fn connect(url: String, config: HashMap<String, String>) -> Result<Self, Error> {
        Ok(Self {
            client: Client::with_uri_str(url.as_str()).await?,
            config,
        })
    }

    fn user_collection(&self) -> Result<Collection<User>, Error> {
        let database_name = self
            .config
            .get("user_db")
            .ok_or(Error::DatabaseNotConfigured)?;

        let collection_name = self
            .config
            .get("user_collection")
            .ok_or(Error::DatabaseNotConfigured)?;

        Ok(self
            .client
            .database(database_name)
            .collection_with_type(collection_name))
    }

    fn create_api_key(&self, salt: &str) -> String {
        Uuid::new_v5(&Uuid::NAMESPACE_OID, salt.as_bytes())
            .as_simple()
            .to_string()
    }

    pub async fn insert_user(
        &self,
        user: NewUser,
        ip: String,
        creator: String,
    ) -> Result<User, Error> {
        let datetime = Utc::now();

        let user = User {
            created_ip: ip,
            created_by: creator,
            created_at: datetime,
            email: user.email,
            description: user.description,
            api_key: self.create_api_key(&datetime.to_string()),
            is_admin: user.is_admin,
            acl_allow_ips: user.acl_allow_ips,
            acl_allow_endpoints: user.acl_allow_endpoints,
        };

        let user_collection = self.user_collection()?;

        if user_collection
            .find_one(doc! { "email": &user.email }, None)
            .await?
            .is_some()
        {
            return Err(Error::UserConflict);
        }

        user_collection
            .insert_one(user.clone(), None)
            .await
            .map(|_| user)
            .map_err(Into::into)
    }

    pub async fn update_user(&self, user: NewUser) -> Result<User, Error> {
        use mongodb::options::{FindOneAndUpdateOptions, ReturnDocument};

        self.user_collection()?
            .find_one_and_update(
                doc! { "email": &user.email},
                doc! {
                    "$set": {
                        "description": user.description,
                        "is_admin": user.is_admin,
                        "acl_allow_endpoints": user.acl_allow_endpoints,
                        "acl_allow_ips": user.acl_allow_ips
                    }
                },
                FindOneAndUpdateOptions::builder()
                    .return_document(Some(ReturnDocument::After))
                    .build(),
            )
            .await?
            .ok_or(Error::NotFound)
    }

    pub async fn get_user_from_api_key(&self, api_key: &str) -> Result<User, Error> {
        self.user_collection()?
            .find_one(doc! {"api_key": api_key}, None)
            .await?
            .ok_or(Error::NotFound)
    }

    pub async fn get_all_users(&self) -> Result<Vec<User>, Error> {
        use futures::stream::TryStreamExt;

        self.user_collection()?
            .find(None, None)
            .await?
            .try_collect()
            .await
            .map_err(Into::into)
    }

    pub async fn delete_user(&self, email: String) -> Result<(), Error> {
        self.user_collection()?
            .delete_one(doc! {"email": email}, None)
            .await
            .map_err(Into::into)
            .and_then(|count| match count {
                DeleteResult { deleted_count, .. } if deleted_count > 0 => Ok(()),
                _ => Err(Error::NotFound),
            })
    }
}
