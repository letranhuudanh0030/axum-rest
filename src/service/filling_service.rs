use crate::model::filling_model::FillingModel;
use crate::repository::filling_repository::AbstractFillingRepository;
use crate::utils::api_response::Error;

pub struct FillingService<'a, T>
where
    T: AbstractFillingRepository + Send + Sync,
{
    filling_repository: &'a T,
}

impl<'a, T> FillingService<'a, T>
where
    T: AbstractFillingRepository + Send + Sync,
{
    pub fn new(filling_repository: &'a T) -> Self {
        Self { filling_repository }
    }

    pub async fn get_all(&self) -> Result<Vec<FillingModel>, Error> {
        self.filling_repository
            .find_all()
            .await
            .map_err(|_| Error::NO_DATA)
    }
}
