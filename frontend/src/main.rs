use uuid::Uuid;
use zoon::{eprintln, *};

fn main() {
    TODOS.init_lazy();
    start_app("app", root);
}

fn root() -> impl Element {
    Column::new()
        .s(Align::center())
        .s(Gap::new().y(32))
        .s(Height::fill())
        .s(Width::fill().max(512))
        .s(Padding::all(32))
        .item(header())
        .item_signal(TODOS.is_empty.signal().map_true(|| {
            El::new().s(Height::fill()).s(AlignContent::center()).child(
                Paragraph::new()
                    .content("Empty")
                    .s(Font::new().color(color!("#aaa8"))),
            )
        }))
        .item_signal(TODOS.is_empty.signal().map_false(|| Spacer::fill()))
        .item(todos())
        .item(input())
}

fn header() -> impl Element {
    Column::new()
        .item(
            Paragraph::new().content("2 2").s(Font::new()
                .color(color!("#3abd69"))
                .weight(FontWeight::ExtraHeavy)
                .size(48)
                .center()),
        )
        .item(
            Paragraph::new().content("TWO DUE - TODO app").s(Font::new()
                .color(color!("#fff"))
                .weight(FontWeight::SemiBold)
                .size(20)
                .center()),
        )
}

fn input() -> impl Element {
    Row::new()
        .s(Gap::new().x(4))
        .s(Width::fill())
        .item(
            TextInput::new()
                .s(Width::fill())
                .s(Padding::new().x(16).y(8))
                .s(Background::new().color(color!("#aaa4")))
                .s(RoundedCorners::all(12))
                .s(Font::new().color(color!("#fff")))
                .focus(true)
                .on_change(|title| TODOS.new_todo.set(title))
                .label_hidden("new todo")
                .placeholder(Placeholder::new("new todo").s(Font::new().color(color!("#fff8"))))
                .text_signal(TODOS.new_todo.signal_cloned()),
        )
        .item(
            Button::new()
                .s(Padding::new().x(16).y(8))
                .s(Background::new().color(color!("#fff")))
                .s(RoundedCorners::all(12))
                .label(
                    Paragraph::new()
                        .content("ADD")
                        .s(Font::new().weight(FontWeight::Bold)),
                )
                .on_press(|| {
                    let mut new = TODOS.new_todo.lock_mut();
                    let title = new.trim();
                    if !title.is_empty() {
                        TODOS.todos.lock_mut().push_cloned({
                            let todo = Todo::new();
                            todo.title.set(title.to_owned());
                            todo
                        });
                        new.clear();
                    }
                }),
        )
}

fn todos() -> impl Element {
    Column::new()
        .s(Width::fill())
        .s(Align::center())
        .item_signal(TODOS.is_empty.signal().map_false(|| {
            Column::new()
                .s(Width::fill())
                .s(Gap::new().y(8))
                .items_signal_vec(
                    TODOS
                        .todos
                        .signal_vec_cloned()
                        .filter_signal_cloned(|todo| {
                            map_ref! {
                                let _ = todo.done.signal() => true
                            }
                        })
                        .map(todo),
                )
        }))
}

fn todo(todo: Todo) -> impl Element {
    Row::new()
        .s(Width::fill())
        .s(Padding::all(8))
        .s(Background::new().color(color!("#121212")))
        .s(RoundedCorners::all(12))
        .s(Gap::new().x(8))
        .update_raw_el(|row| row.class("space-between"))
        .item(todo_title(todo.clone()))
        .item_signal(
            todo.done
                .signal()
                .map_true(move || todo_trash(todo.clone())),
        )
}

fn todo_title(todo: Todo) -> impl Element {
    Row::new()
        .s(Gap::new().x(12))
        .item(todo_checkbox(todo.clone()))
        .item(
            Paragraph::new()
                .s(Width::fill())
                .s(Font::new()
                    .color_signal(
                        todo.done
                            .signal()
                            .map_bool(|| color!("#aaaa"), || color!("#fff")),
                    )
                    .line(FontLine::new().strike_signal(todo.done.signal())))
                .content(&todo.title.get_cloned()),
        )
}

fn todo_checkbox(todo: Todo) -> impl Element {
    static ACTIVE_ICON: &str = "data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHZpZXdCb3g9IjAgMCA0OC41NTUgNDQuMjM2Ij48cGF0aCBmaWxsPSIjZmZmIiBzdHJva2U9IiNmZmYiIGQ9Ik0xMSA0MC45MDZoMjYuNTU1YzMuNTg4IDAgNS41MjEtMS45NTUgNS41MjEtNS41MjFWOC44NTJjMC0zLjU2Ny0xLjkzMy01LjUyMi01LjUyMS01LjUyMkgxMWMtMy42MSAwLTUuNTIxIDEuOTEyLTUuNTIxIDUuNTIydjI2LjUzM2MwIDMuNjEgMS45MTIgNS41MjEgNS41MjEgNS41MjF6bS4wNDMtMS4wNTJjLTIuOTY1IDAtNC41MTItMS41NDctNC41MTItNC41MTJWOC44OTVjMC0yLjk4NyAxLjU0Ny00LjUxMiA0LjUxMi00LjUxMkgzNy40OWMyLjg4IDAgNC41MzMgMS41MjUgNC41MzMgNC41MTJ2MjYuNDQ3YzAgMi45NjUtMS42NTQgNC41MTItNC41MzMgNC41MTJ6Ii8+PC9zdmc+";
    static COMPLETED_ICON: &str = "data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHZpZXdCb3g9IjAgMCA0OC41NTUgNDQuMjM2Ij48cGF0aCBmaWxsPSIjZmZmIiBzdHJva2U9IiNmZmYiIGQ9Ik0xMSA0MC45MDZoMjYuNTU1YzMuNTg4IDAgNS41MjEtMS45NTUgNS41MjEtNS41MjFWOC44NTJjMC0zLjU2Ny0xLjkzMy01LjUyMi01LjUyMS01LjUyMkgxMWMtMy42MSAwLTUuNTIxIDEuOTEyLTUuNTIxIDUuNTIydjI2LjUzM2MwIDMuNjEgMS45MTIgNS41MjEgNS41MjEgNS41MjF6bS4wNDMtMS4wNTJjLTIuOTY1IDAtNC41MTItMS41NDctNC41MTItNC41MTJWOC44OTVjMC0yLjk4NyAxLjU0Ny00LjUxMiA0LjUxMi00LjUxMkgzNy40OWMyLjg4IDAgNC41MzMgMS41MjUgNC41MzMgNC41MTJ2MjYuNDQ3YzAgMi45NjUtMS42NTQgNC41MTItNC41MzMgNC41MTJ6bTEwLjM1NS03LjA5Yy4xOTQgMCAuNDA5LS4wODYuNTgtLjMyM2wxMi4yMDQtMTguODJjLjA2NC0uMTI5LjEyOS0uMy4xMjktLjQzIDAtLjM2NS0uMzIzLS42MDEtLjYyMy0uNjAxLS4xOTQgMC0uNDA5LjEyOS0uNTYuMzQ0TDIxLjM1NiAzMS4xMDlsLTYuODc1LTcuODg0Yy0uMTcxLS4xNzItLjMyMi0uMjU4LS41NTgtLjI1OC0uMjU4IDAtLjYyMy4yMTUtLjYyMy42MjNhLjg0Ljg0IDAgMCAwIC4yMTUuNTE1bDcuMzA0IDguMzU4Yy4xOTQuMjM2LjM2Ni4zLjU4LjN6Ii8+PC9zdmc+";

    Checkbox::new()
        .id(todo.id.to_string())
        .checked_signal(todo.done.signal())
        .on_change(move |done| todo.done.set(done))
        .icon(|done| {
            El::new()
                .s(Width::exact(24))
                .s(Height::exact(24))
                .s(Background::new()
                    .url_signal(done.signal().map_bool(|| COMPLETED_ICON, || ACTIVE_ICON)))
        })
}

fn todo_trash(todo: Todo) -> impl Element {
    static TRASH_ICON: &str = "data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHZpZXdCb3g9IjAgMCA0Ni42NDMgNTguODAzIj48cGF0aCBmaWxsPSIjZmZmIiBzdHJva2U9IiNmZmYiIGQ9Im0zNi45NTMgNDguNzcgMS43ODMtMzMuNzNoNC4xMDRjLjMgMCAuNTE1LS4xOTQuNTE1LS40OTUgMC0uMjgtLjIxNC0uNDk0LS41MTUtLjQ5NEgzMS45OXYtMy42OTZjMC0yLjM0MS0xLjgyNi00LjE0Ni00LjQ5LTQuMTQ2aC04LjM1N2MtMi42NjQgMC00LjQ3IDEuODA1LTQuNDcgNC4xNDZ2My42OTZIMy44MjVjLS4zIDAtLjUzNy4yMzYtLjUzNy40OTQgMCAuMy4yMzYuNDk0LjUzNy40OTRINy45NWwxLjgwNSAzMy43M2MuMTA3IDIuMDg1IDEuOTEyIDMuODI1IDQuMDYgMy44MjVIMzIuODVjMi4xNDggMCAzLjk5Ni0xLjc0IDQuMTAzLTMuODI0ek0xNS43MjcgMTAuMzU1YzAtMS43ODMgMS4zNTMtMy4xMzYgMy40MTYtMy4xMzZIMjcuNWMyLjA2MyAwIDMuNDM4IDEuMzUzIDMuNDM4IDMuMTM2djMuNjk2SDE1LjcyNnptLTEuODkxIDQxLjIyOWMtMS42MzMgMC0yLjk4Ni0xLjMzMi0zLjA3Mi0yLjlMOS4wMDIgMTUuMDM5aDI4LjY4MmwtMS43NjIgMzMuNjQ1Yy0uMDg2IDEuNTY4LTEuNDQgMi45LTMuMDUgMi45em0xNi4wMDYtNS4wOTJjLjMgMCAuNTM3LS4yMTUuNTM3LS40OTRsLjc5NS0yNS4zM2MwLS4yOC0uMjE1LS41MzctLjQ5NC0uNTM3LS4zMDEgMC0uNTE2LjIxNS0uNTE2LjQ5NGwtLjc5NSAyNS4zMDljMCAuMy4xOTQuNTU4LjQ3My41NTh6bS0xMy4wMiAwYy4yOCAwIC40OTQtLjI1OC40OTQtLjU1OEwxNi41IDIwLjYyNWMwLS4yOC0uMjM2LS40OTQtLjUzNy0uNDk0LS4yNTggMC0uNDczLjIxNS0uNDczLjUzN2wuODE3IDI1LjMzYzAgLjI4LjIxNC40OTQuNTE1LjQ5NHptNi41MzIgMGMuMjU3IDAgLjQ5NC0uMjU4LjQ5NC0uNTE1di0yNS4zM2MwLS4zMDEtLjIxNS0uNTE2LS40OTQtLjUxNmEuNTA5LjUwOSAwIDAgMC0uNTE2LjUxNXYyNS4zM2MwIC4yNTguMjM2LjUxNi41MTYuNTE2eiIvPjwvc3ZnPg==";

    Button::new()
        .label(
            El::new()
                .s(Width::exact(24))
                .s(Height::exact(24))
                .s(Background::new().url(TRASH_ICON)),
        )
        .on_press(move || TODOS.todos.lock_mut().retain(|t| t.id != todo.id))
}

const TODOS_KEY: &str = "two-due-22";

static TODOS: Lazy<Todos> = Lazy::new(|| {
    let todos = Todos::default();
    if let Some(Ok(local_todos)) = local_storage().get(TODOS_KEY) {
        todos.todos.lock_mut().replace_cloned(local_todos);
    }
    create_triggers();
    todos
});

#[derive(Default)]
struct Todos {
    todos: MutableVec<Todo>,
    new_todo: Mutable<String>,
    todo_count: Mutable<usize>,
    is_empty: Mutable<bool>,
}

#[derive(Educe)]
#[educe(Deref, Default(new))]
#[derive(PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
#[serde(crate = "serde")]
pub struct TodoId(#[educe(Default(expression = Uuid::new_v4()))] Uuid);

#[derive(Educe)]
#[educe(Default(new))]
#[derive(Deserialize, Serialize, Clone)]
#[serde(crate = "serde")]
struct Todo {
    id: TodoId,
    title: Mutable<String>,
    done: Mutable<bool>,
}

fn create_triggers() {
    Task::start(async {
        TODOS
            .todos
            .signal_vec_cloned()
            .map_signal(|todo| {
                map_ref! {
                    let _ = todo.title.signal_ref(|_| ()),
                    let _ = todo.done.signal_ref(|_| ()),
                    => todo.clone()
                }
            })
            .to_signal_cloned()
            .for_each_sync(|todos| {
                if let Err(err) = local_storage().insert(TODOS_KEY, &todos) {
                    eprintln!("failed to store todos: {err:#?}");
                }
                TODOS.todo_count.set(todos.len());
                TODOS.is_empty.set(todos.is_empty());
            })
            .await
    })
}
