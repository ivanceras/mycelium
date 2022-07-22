//! just add some entries into the forum
#![deny(warnings)]
use codec::Decode;
use frame_support::pallet_prelude::ConstU32;
use frame_support::BoundedVec;
use mycelium::Api;
use sp_core::sr25519::Pair;
use sp_keyring::AccountKeyring;

const DELAY: u64 = 1500; // in ms
type MaxContentLength = ConstU32<280>;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let entries: Vec<(&str,Vec<(&str, Vec<&str>)>)> = vec![
        ("This is content1",
            vec![
                ("This is comment1 of content1",vec![]),
                ("This is comment2 of content1",vec!["This is reply of comment2 of content1"]),
                ("This is comment3 of content1",vec![]),
            ]
        ),

        ("I’d just like to interject for a moment.\
         \nWhat you’re refering to as Linux, is in fact, GNU/Linux, or as I’ve recently taken to calling it, GNU plus Linux. ",
        vec![
            ("Linux is not an operating system unto itself,\
             \nbut rather another free component of a fully functioning GNU system made useful by the GNU corelibs,\
             \nshell utilities and vital system components comprising a full OS as defined by POSIX.",
             vec![]
            ),
            ("Many computer users run a modified version of the GNU system every day, without realizing it. ",
             vec![]
            ),
            ("Through a peculiar turn of events, the version of GNU which is widely used today is often called Linux,\
             and many of its users are not aware that it is basically the GNU system, developed by the GNU Project.",
             vec![]
            ),
            ("There really is a Linux, and these people are using it, but it is just a part of the system they use.\
             \nLinux is the kernel: the program in the system that allocates the machine’s resources to the other programs that you run.",
             vec![]
             ),
             ("The kernel is an essential part of an operating system,\
              \nbut useless by itself; it can only function in the context of a complete operating system.",
              vec![]
             ),
             ("Linux is normally used in combination with the GNU operating system:\
              \nthe whole system is basically GNU with Linux added, or GNU/Linux.",
             vec![]
             ),
             ("All the so-called Linux distributions are really distributions of GNU/Linux!",
             vec![]
            ),
        ]),

        ("Thou TCP/IP ensures the delivery and acknowledge,\
        \nbut UDP sacrifice accuracy for speed for applications such as games and movies, users don't want to wait\
        \n-- Sun Tzu, 1337 AD",
        vec![
        ]),

        ("Shakespeare quote of the Day:\
        \nAn SSL error has occured and a secure connection to the server cannot be made.",
         vec![
         ("Bruh",vec![]),
         ]
        ),

        ("His palms are sweaty\
        \nKnees weak, arms are heavy\
        \nThe unit tests are failing already\
        \nCode spaghetti",
        vec![
        ("He's nervous,\
            \nBut at his laptop he looks calm and ready\
            \nTo squash bugs\
            \nBut he keeps on forgetting",
            vec![]),

            ("What he typed out\
            \nThe key taps grow so loud\
            \nHe checks his commits\
            \nBut the logs won’t turn out\
            \nHe’s spacing, how\
            \nEverybody’s pacing now\
            \nThe clock’s run out, deadline\
            \nIt’s due now!",
            vec![]
            ),


            ("Snap back to the IDE,\
            \nOh, there goes TDD\
            \nOh there goes habits he knows\
            \nHe’s so mad but he goes\
            \nDeeper in debt that easy\
            \nNo, he won’t have it\
            \nHe knows, his old build server\
            \nWoke, he knows his whole build will be broke\
            \nIt don’t matter, he’ll cope",
            vec![]
            ),
            ]
        ),
    ];

    let api = Api::new("http://localhost:9933").await?;
    let alice: sp_core::sr25519::Pair = AccountKeyring::Alice.pair();
    let bob: sp_core::sr25519::Pair = AccountKeyring::Bob.pair();

    for (post, replies0) in entries {
        println!("post: {}", post);
        let post_id = add_post(&api, post, &alice).await?;
        sleep(DELAY);
        for (reply, replies1) in replies0 {
            println!("\t>{}", reply);
            let comment_id = add_comment_to(&api, post_id, reply, &bob).await?;
            sleep(DELAY);
            for reply in replies1 {
                println!("\t\t>{}", reply);
                let _comment_id = add_comment_to(&api, comment_id, reply, &alice).await?;
                sleep(DELAY);
            }
        }
    }
    Ok(())
}

fn sleep(ms: u64) {
    std::thread::sleep(std::time::Duration::from_millis(ms));
}

async fn add_post(api: &Api, post: &str, author: &Pair) -> Result<u32, mycelium::Error> {
    println!("post len: {}", post.len());
    let pallet = api.metadata().pallet("ForumModule")?;
    let call_index = pallet
        .calls
        .get("post_content")
        .expect("unable to find function");

    let bounded_content =
        BoundedVec::try_from(post.as_bytes().to_vec()).expect("Content is too long");
    let call: ([u8; 2], BoundedVec<u8, MaxContentLength>) =
        ([pallet.index, *call_index], bounded_content);

    let current_item = get_current_item(api).await?;

    let extrinsic = api.sign_extrinsic(author.clone(), call).await?;
    let result = api.submit_extrinsic(extrinsic).await?;
    println!("result: {:?}", result);

    Ok(current_item)
}

/// Warning this is an approximation value, since
/// there could another extrinsic call to the forum module to increment it while
/// this is executing in between the function calls in the following intended extrinsics
async fn get_current_item(api: &Api) -> Result<u32, mycelium::Error> {
    let current_item: Option<Vec<u8>> = api
        .fetch_opaque_storage_value("ForumModule", "ItemCounter")
        .await?;

    if let Some(current_item) = current_item {
        let current_item = Decode::decode(&mut current_item.as_slice())?;
        Ok(current_item)
    } else {
        println!("There is no current item yet..");
        eprintln!("There is no current item");
        Ok(0)
    }
}

async fn add_comment_to(
    api: &Api,
    parent_item: u32,
    comment: &str,
    author: &Pair,
) -> Result<u32, mycelium::Error> {
    println!("comment len: {}", comment.len());
    let pallet = api.metadata().pallet("ForumModule")?;
    let call_index = pallet.calls.get("comment_on").unwrap();
    let bounded_comment =
        BoundedVec::try_from(comment.as_bytes().to_vec()).expect("Content is too long");
    let call: ([u8; 2], u32, BoundedVec<u8, MaxContentLength>) =
        ([pallet.index, *call_index], parent_item, bounded_comment);

    let current_item = get_current_item(api).await?;

    let extrinsic = api.sign_extrinsic(author.clone(), call).await?;
    let result = api.submit_extrinsic(extrinsic).await?;
    println!("comment result: {:?}", result);

    Ok(current_item)
}
