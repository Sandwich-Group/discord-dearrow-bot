pub async fn fetch_new_title(video_id: &str) -> Result<Option<String>, Box<dyn std::error::Error>> {
    let url = format!("https://sponsor.ajay.app/api/branding?videoID={video_id}&service=YouTube");

    let resp = reqwest::get(url).await?.json::<BrandingResponse>().await?;

    let Some(top_title) = resp.titles.into_iter().next() else {
        // no titles found. shouldn't happen, the original title should be in the response.
        return Ok(None);
    };

    if !(top_title.locked || top_title.votes >= 0) || top_title.original {
        // no satisfactory titles found
        return Ok(None);
    }

    Ok(Some(top_title.title))
}

#[derive(serde::Deserialize)]
struct BrandingResponse {
    titles: Vec<BrandingTitle>,
    // thumbnails: Vec<()>,
    // random_time: i64,
    // video_duration: Option<i64>,
}

#[derive(serde::Deserialize)]
struct BrandingTitle {
    title: String,
    original: bool,
    votes: i64,
    locked: bool,
    // uuid: String,
    // user_id: Option<String>, // only if requested
}
