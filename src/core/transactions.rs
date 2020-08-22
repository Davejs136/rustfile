enum TransactionError {
  GetDataFailed,
  PostDataFailed,
  ErrorToDelete,
  CannotUpdate,
}
/* 
  get
  post
  delete
  put

  get(name) -> result
  post(name, data) -> result(new data)
  delete(id) -> 
*/

type TransactionResult = Result<(), TransactionError>;

trait Actions<T> {
  // Get a data with the name
  fn get(&self, name: String) -> TransactionResult;

  // Create new data
  fn post(&self, name: String, data: T) -> TransactionResult;

  // Delete existing data
  fn delete(&self, id: String) -> TransactionResult;

  // Update a data
  fn put(&self, old: String, new: T) -> TransactionResult;
}

struct Transaction;

impl<T: std::fmt::Debug> Actions<T> for Transaction {
  fn get(&self, name: String) -> TransactionResult {
    dbg!(name);
    
    Ok(())
  }

  fn post(&self, name: String, data: T) -> TransactionResult {
    dbg!(name);
    
    Ok(())
  }

  fn delete(&self, id: String) -> TransactionResult {
    dbg!(id);
    
    Ok(())
  }

  fn put(&self, old: String, new: T) -> TransactionResult {
    dbg!(old, new);
    
    Ok(())
  }

}

#[test]
fn check_get_transaction() {
    let actions = Actions;
    assert_eq!(Ok(()), actions.get("Todo en orden"));
}