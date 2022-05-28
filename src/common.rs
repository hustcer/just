// stdlib
pub(crate) use std::{
  cmp,
  collections::{BTreeMap, BTreeSet},
  env,
  ffi::{OsStr, OsString},
  fmt::{self, Debug, Display, Formatter},
  fs,
  io::{self, Cursor, Write},
  iter::{self, FromIterator},
  mem,
  ops::{Index, Range, RangeInclusive},
  path::{self, Path, PathBuf},
  process::{self, Command, ExitStatus, Stdio},
  rc::Rc,
  str::{self, Chars},
  sync::{Mutex, MutexGuard},
  usize, vec,
};

// dependencies
pub(crate) use ::{
  camino::Utf8Path,
  derivative::Derivative,
  edit_distance::edit_distance,
  lexiclean::Lexiclean,
  libc::EXIT_FAILURE,
  log::{info, warn},
  regex::Regex,
  serde::{
    ser::{SerializeMap, SerializeSeq},
    Serialize, Serializer,
  },
  snafu::{ResultExt, Snafu},
  strum::{Display, EnumString, IntoStaticStr},
  typed_arena::Arena,
  unicode_width::{UnicodeWidthChar, UnicodeWidthStr},
};

// modules
pub(crate) use crate::{completions, config, config_error, keyed};

// functions
pub(crate) use crate::{load_dotenv::load_dotenv, output::output, unindent::unindent};

// traits
pub(crate) use crate::{
  color_display::ColorDisplay, command_ext::CommandExt, keyed::Keyed, ordinal::Ordinal,
  platform_interface::PlatformInterface, range_ext::RangeExt,
};

// structs and enums
pub(crate) use crate::{
  alias::Alias, analyzer::Analyzer, assignment::Assignment,
  assignment_resolver::AssignmentResolver, ast::Ast, binding::Binding, color::Color,
  compile_error::CompileError, compile_error_kind::CompileErrorKind,
  conditional_operator::ConditionalOperator, config::Config, config_error::ConfigError,
  count::Count, delimiter::Delimiter, dependency::Dependency, dump_format::DumpFormat,
  enclosure::Enclosure, error::Error, evaluator::Evaluator, expression::Expression,
  fragment::Fragment, function::Function, function_context::FunctionContext,
  interrupt_guard::InterruptGuard, interrupt_handler::InterruptHandler, item::Item,
  justfile::Justfile, keyword::Keyword, lexer::Lexer, line::Line, list::List, loader::Loader,
  name::Name, output_error::OutputError, parameter::Parameter, parameter_kind::ParameterKind,
  parser::Parser, platform::Platform, position::Position, positional::Positional, recipe::Recipe,
  recipe_context::RecipeContext, recipe_resolver::RecipeResolver, scope::Scope, search::Search,
  search_config::SearchConfig, search_error::SearchError, set::Set, setting::Setting,
  settings::Settings, shebang::Shebang, shell::Shell, show_whitespace::ShowWhitespace,
  string_kind::StringKind, string_literal::StringLiteral, subcommand::Subcommand,
  suggestion::Suggestion, table::Table, thunk::Thunk, token::Token, token_kind::TokenKind,
  unresolved_dependency::UnresolvedDependency, unresolved_recipe::UnresolvedRecipe,
  use_color::UseColor, variables::Variables, verbosity::Verbosity, warning::Warning,
};

// type aliases
pub(crate) type CompileResult<'a, T> = Result<T, CompileError<'a>>;
pub(crate) type ConfigResult<T> = Result<T, ConfigError>;
pub(crate) type RunResult<'a, T> = Result<T, Error<'a>>;
pub(crate) type SearchResult<T> = Result<T, SearchError>;

// modules used in tests
#[cfg(test)]
pub(crate) use crate::testing;

// structs and enums used in tests
#[cfg(test)]
pub(crate) use crate::{node::Node, tree::Tree};
