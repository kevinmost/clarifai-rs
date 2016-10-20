use model::{
    ConceptModel,
    ModelData,
};

// The general model, used to predict all concepts that Clarifai is aware of
// pub const GENERAL: String = String::from("");
pub const GENERAL: ConceptModel<'static> = ConceptModel {
    data: ModelData {
        id: "aaa03c23b3724a16a56b629203edc62c",
        name: Some("general-v1.3"),
        created_at: None,
        updated_at: None,
        app_id: None,
        version: None,
    },
};
// pub const GENERAL: Model<'static> = Concept(ConceptData {
// });
