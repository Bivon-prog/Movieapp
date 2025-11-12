# 🎬 StreamBox - Movie Streaming Platform

A modern movie streaming website built with **Rust** backend and **Vanilla JavaScript** frontend, powered by The Movie Database (TMDb) API.

![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)
![JavaScript](https://img.shields.io/badge/JavaScript-F7DF1E?style=for-the-badge&logo=javascript&logoColor=black)
![TailwindCSS](https://img.shields.io/badge/Tailwind_CSS-38B2AC?style=for-the-badge&logo=tailwind-css&logoColor=white)

## ✨ Features

- 🎥 **Browse Movies** - Trending, Popular, Top Rated, and Upcoming
- 🔍 **Global Search** - Search millions of movies from TMDb database
- 📺 **Watch Trailers** - YouTube trailers embedded in modal popups
- 📄 **Pagination** - Browse through thousands of movies
- 🎨 **Beautiful UI** - Netflix-inspired dark theme with Tailwind CSS
- ⚡ **Fast Backend** - Rust-powered API with Actix-web
- 🌐 **Real-time Data** - Live movie data from TMDb API

## 🚀 Quick Start

### Prerequisites

- [Rust](https://rustup.rs/) (latest stable version)
- TMDb API Key (get it free at [themoviedb.org](https://www.themoviedb.org/settings/api))

### Installation

1. **Clone the repository**
   ```bash
   git clone https://github.com/yourusername/streambox.git
   cd streambox
   ```

2. **Set up environment variables**
   
   Create a `.env` file in the root directory:
   ```env
   TMDB_API_KEY=your_api_key_here
   TMDB_ACCESS_TOKEN=your_access_token_here
   TMDB_BASE_URL=https://api.themoviedb.org/3
   ```

3. **Run the application**
   ```bash
   cargo run
   ```

4. **Open your browser**
   ```
   http://localhost:8080
   ```

That's it! 🎉

## 📁 Project Structure

```
streambox/
├── src/
│   └── main.rs              # Rust backend server
├── public/
│   ├── index.html           # Main HTML page
│   ├── app.js               # JavaScript logic
│   ├── style.css            # Custom styles
│   └── test.html            # API test page
├── Cargo.toml               # Rust dependencies
├── .env                     # Environment variables (not in git)
├── .gitignore              # Git ignore rules
└── README.md               # This file
```

## 🎯 API Endpoints

| Endpoint | Method | Description |
|----------|--------|-------------|
| `/api/health` | GET | Health check |
| `/api/movies?page=1&category=trending` | GET | Get movies by category |
| `/api/search?q=query` | GET | Search movies |
| `/api/movies/{id}` | GET | Get movie details |
| `/api/movies/{id}/videos` | GET | Get movie trailers |

### Categories
- `trending` - Trending movies this week
- `popular` - Popular movies
- `top_rated` - Top rated movies
- `upcoming` - Upcoming releases
- `now_playing` - Now playing in theaters

## 🛠️ Technologies

### Backend
- **Rust** - Systems programming language
- **Actix-web** - Fast web framework
- **Reqwest** - HTTP client for TMDb API
- **Serde** - JSON serialization
- **Dotenv** - Environment variable management

### Frontend
- **Vanilla JavaScript** - No frameworks, pure JS
- **Tailwind CSS** - Utility-first CSS framework
- **YouTube Embed API** - For trailer playback

## 🎨 Features in Detail

### Search
- Real-time search across TMDb's entire database
- Debounced input (500ms) for better performance
- Shows result count
- Searches by title, genre, and description

### Movie Categories
- **Home** - Trending movies of the week
- **Movies** - Popular movies
- **Top Rated** - Highest rated films
- Browse by genre (Action, Drama, Comedy, Thriller, Sci-Fi)

### Video Player
- Click any movie to see details
- Watch trailers in embedded YouTube player
- View ratings, release year, and description
- Direct link to TMDb page

### Pagination
- 20 movies per page
- Navigate through multiple pages
- Smooth loading experience

## 🔧 Development

### Run in development mode
```bash
cargo run
```

### Build for production
```bash
cargo build --release
```

The optimized binary will be in `target/release/`

### Test the API
Visit `http://localhost:8080/test.html` to test API endpoints

## 📝 Environment Variables

| Variable | Description | Required |
|----------|-------------|----------|
| `TMDB_API_KEY` | Your TMDb API key | Yes |
| `TMDB_ACCESS_TOKEN` | Your TMDb access token | Yes |
| `TMDB_BASE_URL` | TMDb API base URL | No (defaults to official URL) |

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the project
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- [The Movie Database (TMDb)](https://www.themoviedb.org/) - For providing the movie data API
- [Actix-web](https://actix.rs/) - For the excellent Rust web framework
- [Tailwind CSS](https://tailwindcss.com/) - For the utility-first CSS framework

## 📧 Contact

Your Name - [@yourtwitter](https://twitter.com/yourtwitter)

Project Link: [https://github.com/yourusername/streambox](https://github.com/yourusername/streambox)

---

⭐ Star this repo if you find it helpful!
