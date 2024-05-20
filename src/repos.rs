use crate::repository::files::{Files, FilesRepository, MockFiles};

pub trait OwnRepositories {
    type FilesRepository: FilesRepository;

    fn files(&self) -> Self::FilesRepository;
}

pub struct Repos {}
impl OwnRepositories for Repos {
    type FilesRepository = Files;

    fn files(&self) -> Self::FilesRepository {
        Files {}
    }
}

pub struct MockRepo {}
impl OwnRepositories for MockRepo {
    type FilesRepository = MockFiles;

    fn files(&self) -> Self::FilesRepository {
        MockFiles {}
    }
}
