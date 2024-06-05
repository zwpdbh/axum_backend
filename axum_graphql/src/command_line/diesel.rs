use clap::Subcommand;

#[derive(Subcommand, Debug, Clone)]
pub enum AcstorDemo {
    Show,
    Create {
        #[arg(long)]
        title: String,
        #[arg(long)]
        body: String,
    },
    Update {
        #[arg(long)]
        id: i32,
    },
    Select {
        #[arg(long)]
        id: i32,
    },
    Delete {
        #[arg(long)]
        target: String,
    },
}

#[derive(Subcommand, Debug, Clone)]
pub enum PostDemo {
    Show,
    Create {
        #[arg(long)]
        title: String,
        #[arg(long)]
        body: String,
    },
    Update {
        #[arg(long)]
        id: i32,
    },
    Select {
        #[arg(long)]
        id: i32,
    },
    Delete {
        #[arg(long)]
        target: String,
    },
}

#[derive(Debug, Clone, Subcommand)]
pub enum DieselDemoCase {
    Post {
        #[clap(subcommand)]
        demo: PostDemo,
    },
    Acstor {
        #[clap(subcommand)]
        demo: AcstorDemo,
    },
}

pub fn run(case: DieselDemoCase) {
    match case {
        DieselDemoCase::Post { demo } => match demo {
            PostDemo::Show => diesel_demo::models::post::show_post(),
            PostDemo::Create { title, body } => {
                diesel_demo::models::post::create_post(&title, &body)
            }
            PostDemo::Update { id } => diesel_demo::models::post::update_post(id),
            PostDemo::Select { id } => diesel_demo::models::post::select_post(id),
            PostDemo::Delete { target } => diesel_demo::models::post::delete_post(&target),
        },
        DieselDemoCase::Acstor { demo } => match demo {
            _ => todo!("not tried yet"),
        },
    }
}
