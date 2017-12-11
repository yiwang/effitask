use gtk;
use gtk::prelude::*;
use relm_attributes::widget;
use widgets::tasks::Msg::Complete;

#[derive(Msg)]
pub enum Msg {
    Complete(::tasks::Task),
    Filter(Option<String>),
    UpdateFilters(Vec<(String, u32)>),
    UpdateTasks(Vec<::tasks::Task>),
}

#[repr(u32)]
enum Column {
    Title = 0,
    Raw = 1,
    Progress = 2,
}

impl ::std::convert::Into<u32> for Column
{
    fn into(self) -> u32
    {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

impl ::std::convert::Into<i32> for Column
{
    fn into(self) -> i32
    {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

impl Filter
{
    fn update_filters(&mut self, filters: Vec<(String, u32)>)
    {
        let selection = self.filters.get_selection();
        let (paths, _) = selection.get_selected_rows();

        self.model.clear();
        let mut root = ::std::collections::HashMap::new();

        for filter in filters {
            self.append(&mut root, filter);
        }

        self.filters.expand_all();

        for path in paths {
            self.filters.set_cursor(&path, None, false);
        }
    }

    fn append(&self, root: &mut ::std::collections::HashMap<String, ::gtk::TreeIter>, filter: (String, u32))
    {
        use gtk::ToValue;
        use std::slice::SliceConcatExt;

        let (filter, progress) = filter;
        let f = filter.clone();

        let mut levels: Vec<_> = f.split("-")
            .collect();
        let title = levels.pop()
            .unwrap();

        let parent = levels.join("-");

        if parent.len() > 0 && root.get(&parent).is_none() {
            self.append(root, (parent.clone(), 0));
        }

        let row = self.model.append(root.get(&parent));

        self.model.set_value(&row, Column::Title.into(), &title.to_value());
        self.model.set_value(&row, Column::Raw.into(), &filter.to_value());
        self.model.set_value(&row, Column::Progress.into(), &progress.to_value());

        root.insert(filter, row);
    }

    fn update_tasks(&self, tasks: Vec<::tasks::Task>)
    {
        self.tasks.emit(::widgets::tasks::Msg::Update(tasks));
    }
}

#[widget]
impl ::relm::Widget for Filter
{
    fn init_view(&mut self)
    {
        self.filters.set_size_request(200, -1);
        self.scroll.set_policy(::gtk::PolicyType::Never, ::gtk::PolicyType::Automatic);
        self.filters.set_model(Some(&self.model));

        let column = ::gtk::TreeViewColumn::new();
        self.filters.append_column(&column);

        let cell = ::gtk::CellRendererProgress::new();
        cell.set_property_text_xalign(0.);
        column.pack_start(&cell, true);
        column.add_attribute(&cell, "text", Column::Title.into());
        column.add_attribute(&cell, "value", Column::Progress.into());
    }

    fn model(_: ()) -> ::gtk::TreeStore
    {
        let columns = vec![
            ::gtk::Type::String,
            ::gtk::Type::String,
            ::gtk::Type::U32,
        ];

        ::gtk::TreeStore::new(&columns)
    }

    fn update(&mut self, event: Msg)
    {
        use self::Msg::*;

        match event {
            Complete(_) => (),
            Filter(_) => (),
            UpdateFilters(filters) => self.update_filters(filters),
            UpdateTasks(tasks) => self.update_tasks(tasks),
        }
    }

    view!
    {
        gtk::Paned {
            orientation: ::gtk::Orientation::Horizontal,
            #[name="scroll"]
            gtk::ScrolledWindow {
                #[name="filters"]
                gtk::TreeView {
                    headers_visible: false,
                    selection.changed(selection) => {
                        if let Some((list_model, iter)) = selection.get_selected() {
                            let filter = list_model.get_value(&iter, Column::Raw.into())
                                .get();

                            Msg::Filter(filter)
                        }
                        else {
                            Msg::Filter(None)
                        }
                    },
                }
            },
            gtk::ScrolledWindow {
                #[name="tasks"]
                ::widgets::Tasks {
                    Complete(ref task) => Msg::Complete(task.clone()),
                },
            }
        }
    }
}
