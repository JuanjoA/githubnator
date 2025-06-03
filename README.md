# GitHub Issue Finder

A simple, privacy-focused web application to search and filter GitHub issues and pull requests across multiple repositories.


## Features

- Search across multiple GitHub repositories simultaneously
- Filter by issue/PR text, author, label, and assignee
- Switch easily between issues and pull requests
- Privacy guaranteed - all processing happens in your browser
- No data is stored or sent to any server
- Responsive design that works on both desktop and mobile

## How It Works

1. **Add repositories** you want to search through
2. **Choose issues or PRs** using the toggle buttons
3. **Enter your search criteria** in the filter fields:
   - General text search
   - Author name
   - Label
   - Assignee
4. **Click "GO"** to search GitHub with your parameters

The app builds the appropriate GitHub search URL with your filters and opens it in a new tab, taking you directly to GitHub's search results.

## Privacy

This application respects your privacy completely:
- No data storage: We don't save any of your filters or preferences
- Everything stays local: The entire app runs in your browser
- No tracking: We don't use cookies or analyze your behavior

## Development

This project is built with:
- [Rust](https://www.rust-lang.org/)
- [Leptos](https://leptos.dev/) - A Rust framework for building web applications
- [Bulma CSS](https://bulma.io/) - For responsive styling

### Running Locally

```bash
# Clone the repository
git clone https://github.com/juanjoa/github-issue-finder.git
cd github-issue-finder

# Build and run the development server
trunk serve --port 3000

```

Visit `http://localhost:3000` in your browser to use the application.

## License

AGPL-3.0 license