use crate::models;
use dialoguer::{Input, MultiSelect};
use std::fs;

pub async fn fetch_json() -> Result<(), Box<dyn std::error::Error>> {
    let reqwest = reqwest::Client::new();

    // Cookie entry because it requires it to request
    let input: String = Input::new().with_prompt("Enter cookie").interact_text()?;

    let resp = reqwest.get("https://registrationssb.ucr.edu/StudentRegistrationSsb/ssb/classSearch/getTerms?searchTerm=&offset=1&max=10")
        .send().await?
        .json::<Vec<models::Term>>().await?;

    let chosen = MultiSelect::new()
        .with_prompt("\nSelect which terms to download")
        .items(&resp)
        .interact()?;

    for selected in chosen {
        let term = &resp[selected];
        println!("\nDownloading data for term {}", term.code);

        // Create a request client and request with cookie
        let mut c = true;
        let mut offset = 0;
        let mut collected = Vec::new();

        // selects the term, required before fetching data
        reqwest::Client::new().post("https://registrationssb.ucr.edu/StudentRegistrationSsb/ssb/term/search?mode=search")
            .form(&[("term", &term.code)])
            .header("Cookie", &input)
            .send()
            .await?;

        while c {
            // fetch the data and parse it
            let url = format!("https://registrationssb.ucr.edu/StudentRegistrationSsb/ssb/searchResults/searchResults?txt_term={}&pageOffset={}&pageMaxSize=500&sortColumn=subjectDescription&sortDirection=asc", term.code, offset);
            // for some reason making a new client every time makes it error less
            let mut resp = reqwest::Client::new()
                .get(url)
                .header("Cookie", &input)
                .send()
                .await?
                .json::<crate::models::Root>()
                .await?
                .data;

            if resp.len() > 0 {
                if resp[0].term != term.code {
                    panic!("\nError: Term mismatch");
                }
                // appends the new data to the existing data
                collected.append(&mut resp);
                offset += 500;
            } else {
                // stops the loop if there are no more entries
                c = false;
            }

            // Cooldown between requests
            std::thread::sleep(std::time::Duration::from_millis(500));
        }

        // Writes the json to a file in data directory
        fs::create_dir_all("data")?;
        fs::write(
            format!("data/{}.json", term.code),
            serde_json::to_vec(&collected)?,
        )?;

        println!("Successfully downloaded data for term {}", term.description);
    }

    Ok(())
}
