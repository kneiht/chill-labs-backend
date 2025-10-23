use super::model::{Note, NoteRow};
use crate::crud_repository;

crud_repository!(
  NoteRepository,
  Note,
  NoteRow,
  "notes",
  id, user_id, title, content, created, updated;
  id, user_id, title, content, created, updated;
  user_id, title, content, updated;
);
