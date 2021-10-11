use crate::display::App;
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    symbols,
    text::{Span, Spans},
    widgets::canvas::{Canvas, Line, Map, MapResolution},
    widgets::{Block, Borders, List, ListItem, Row, Table, Tabs},
    Frame,
};

pub fn draw<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let chunks = Layout::default()
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(f.size());
    let titles = app
        .tabs
        .titles
        .iter()
        .map(|t| Spans::from(Span::styled(*t, Style::default().fg(Color::Green))))
        .collect();
    let tabs = Tabs::new(titles)
        .block(Block::default().borders(Borders::ALL).title(app.title))
        .highlight_style(Style::default().fg(Color::Yellow))
        .select(app.tabs.index);
    f.render_widget(tabs, chunks[0]);
    match app.tabs.index {
        0 => draw_first_tab(f, app, chunks[1]),
        1 => draw_second_tab(f, app, chunks[1]),
        _ => {}
    };
}

fn draw_first_tab<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
        .direction(Direction::Horizontal)
        .split(area);
    let up_style = Style::default().fg(Color::Green);
    let rows = app.nodes.iter().map(|n| {
        Row::new(vec![n.name.clone(), n.capital.clone(), n.count.to_string()]).style(up_style)
    });
    let table = Table::new(rows)
        .header(
            Row::new(vec!["location", "capital", "count"])
                .style(Style::default().fg(Color::Yellow))
                .bottom_margin(1),
        )
        .block(Block::default().title("nodes").borders(Borders::ALL))
        .widths(&[
            Constraint::Length(20),
            Constraint::Length(20),
            Constraint::Length(20),
        ]);
    f.render_widget(table, chunks[0]);

    let map = Canvas::default()
        .block(Block::default().title("world").borders(Borders::ALL))
        .paint(|ctx| {
            ctx.draw(&Map {
                color: Color::White,
                resolution: MapResolution::High,
            });
            // ctx.layer();
            // for (i, n1) in app.nodes.iter().enumerate() {
            //     for n2 in &app.nodes[i + 1..] {
            //         ctx.draw(&Line {
            //             x1: n1.coordinates[1],
            //             y1: n1.coordinates[0],
            //             y2: n2.coordinates[0],
            //             x2: n2.coordinates[1],
            //             color: Color::Yellow,
            //         });
            //     }
            // }
            for node in &app.nodes {
                ctx.print(node.coordinates[1], node.coordinates[0], "X", Color::Green);
            }
        })
        .marker(if app.enhanced_graphics {
            symbols::Marker::Braille
        } else {
            symbols::Marker::Dot
        })
        .x_bounds([-180.0, 180.0])
        .y_bounds([-90.0, 90.0]);
    f.render_widget(map, chunks[1]);
}

fn draw_second_tab<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(100)].as_ref())
        .split(area);

    // draw raw_nodes list
    let raw_nodes: Vec<ListItem> = app
        .raw_nodes
        .items
        .iter()
        .map(|i| ListItem::new(vec![Spans::from(Span::raw(*i))]))
        .collect();
    let raw_nodes = List::new(raw_nodes)
        .block(Block::default().borders(Borders::ALL).title("nodes"))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD));
    f.render_stateful_widget(raw_nodes, chunks[0], &mut app.raw_nodes.state);
}
