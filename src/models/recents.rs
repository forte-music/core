use crate::context::GraphQLContext;
use crate::database::album;
use crate::database::artist;
use crate::models::*;
use diesel::prelude::*;
use juniper::{FieldResult, GraphQLUnion};

#[derive(GraphQLUnion)]
#[graphql(Context = GraphQLContext)]
pub enum RecentItem {
    Album(Album),
    Artist(Artist),
}

fn merge_recents(albums: Vec<Album>, artists: Vec<Artist>) -> Vec<RecentItem> {
    let recent_albums = albums.into_iter().map(RecentItem::Album);
    let recent_artists = artists.into_iter().map(RecentItem::Artist);

    let recent: Vec<RecentItem> = recent_albums.chain(recent_artists).collect();

    recent
}

fn combine_and_truncate<F, O>(
    albums: Vec<Album>,
    artists: Vec<Artist>,
    first: i64,
    compare_by: F,
) -> Vec<RecentItem>
where
    O: Ord,
    F: Fn(&RecentItem) -> O,
{
    let mut recents = merge_recents(albums, artists);

    recents.sort_by(|a, b| Ord::cmp(&compare_by(b), &compare_by(a)));
    recents.truncate(first as usize);

    recents
}

impl RecentItem {
    pub fn recently_added(context: &GraphQLContext, first: i64) -> FieldResult<Vec<RecentItem>> {
        let conn = &context.connection() as &SqliteConnection;

        let albums: Vec<Album> = album::table
            .order_by(album::time_added.desc())
            .limit(first)
            .load(conn)?;

        let artists: Vec<Artist> = artist::table
            .order_by(artist::time_added.desc())
            .limit(first)
            .load(conn)?;

        Ok(combine_and_truncate(
            albums,
            artists,
            first,
            RecentItem::time_added,
        ))
    }

    pub fn recently_played(context: &GraphQLContext, first: i64) -> FieldResult<Vec<RecentItem>> {
        let conn = &context.connection() as &SqliteConnection;

        let albums: Vec<Album> = album::table
            .filter(album::last_played.is_not_null())
            .order_by(album::last_played.desc())
            .limit(first)
            .load(conn)?;

        let artists: Vec<Artist> = artist::table
            .filter(artist::last_played.is_not_null())
            .order_by(artist::last_played.desc())
            .limit(first)
            .load(conn)?;

        Ok(combine_and_truncate(
            albums,
            artists,
            first,
            RecentItem::last_played,
        ))
    }

    fn time_added(&self) -> NaiveDateTime {
        match self {
            RecentItem::Album(album) => album.time_added,
            RecentItem::Artist(artist) => artist.time_added,
        }
    }

    fn last_played(&self) -> NaiveDateTime {
        match self {
            RecentItem::Album(album) => album.last_played.unwrap(),
            RecentItem::Artist(artist) => artist.last_played.unwrap(),
        }
    }
}
