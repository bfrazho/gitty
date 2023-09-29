use dialoguer::{theme::ColorfulTheme, MultiSelect};

fn main() {
    let repo = gix::discover(".").unwrap();
    let result = repo.find_default_remote(gix::remote::Direction::Fetch).unwrap().unwrap();
    println!("{}", result.name().unwrap().as_bstr());
    //get token for git upon startup, prompt if a local file doesn't exist, then create it next to the executable

    //get collaborators on repository
    //https://api.github.com/repos/git/git/collaborators
    //https://docs.github.com/en/rest/collaborators/collaborators?apiVersion=2022-11-28
    //show collaborator list here
    let multiselected = &[
        "Ice Cream",
        "Vanilla Cupcake",
        "Chocolate Muffin",
        "A Pile of sweet, sweet mustard",
    ];
    let selections = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Pick your food")
        .items(&multiselected[..])
        .interact()
        .unwrap();

    if selections.is_empty() {
        println!("You did not select anything :(");
    } else {
        println!("You selected these things:");
        for selection in selections {
            println!("  {}", multiselected[selection]);
        }
    }
}
