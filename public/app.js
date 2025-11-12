// API Base URL
const API_BASE = '/api';

// State
let allMovies = [];
let currentFilter = { category: 'all', genre: 'all', year: null, search: '' };
let currentPage = 1;
let totalPages = 10; // TMDb has many pages

// Fetch movies from backend
async function fetchMovies(page = 1, category = 'trending') {
    try {
        const response = await fetch(`${API_BASE}/movies?page=${page}&category=${category}`);
        const data = await response.json();
        allMovies = data.movies || [];
        currentPage = page;
        renderMovies();
        setupHero();
        updatePagination();
    } catch (error) {
        console.error('Error fetching movies:', error);
        document.getElementById('loading').textContent = 'Error loading movies. Please try again.';
    }
}

// Setup hero carousel
function setupHero() {
    if (allMovies.length === 0) return;
    
    const heroMovies = allMovies.slice(0, 3);
    let heroIndex = 0;
    
    function updateHero() {
        const movie = heroMovies[heroIndex];
        document.getElementById('hero-img').src = movie.poster;
        document.getElementById('hero-title').textContent = movie.title;
        document.getElementById('hero-sub').textContent = `${movie.genre} • ${movie.year} • ⭐ ${movie.rating}`;
    }
    
    updateHero();
    
    setInterval(() => {
        heroIndex = (heroIndex + 1) % heroMovies.length;
        updateHero();
    }, 4500);
}

// Filter movies (only for local filtering, not search)
function getFilteredMovies() {
    // If we're showing search results, don't filter further
    if (currentFilter.search) {
        return allMovies;
    }
    
    return allMovies.filter(movie => {
        // Genre filter
        if (currentFilter.genre !== 'all' && movie.genre !== currentFilter.genre) {
            return false;
        }
        
        // Year filter
        if (currentFilter.year && movie.year < currentFilter.year) {
            return false;
        }
        
        // Category filter (top rated)
        if (currentFilter.category === 'top' && movie.rating < 8.5) {
            return false;
        }
        
        return true;
    });
}

// Render movies
function renderMovies() {
    const loading = document.getElementById('loading');
    const grid = document.getElementById('movies-grid');
    const filtered = getFilteredMovies();
    
    loading.style.display = 'none';
    
    // Update section title
    let title = 'Popular Movies';
    if (currentFilter.search) {
        title = `Search results for "${currentFilter.search}"`;
    } else if (currentFilter.genre !== 'all') {
        title = `${currentFilter.genre} Movies`;
    } else if (currentFilter.category === 'top') {
        title = 'Top Rated Movies';
    }
    document.getElementById('section-title').textContent = title;
    
    // Clear grid
    grid.innerHTML = '';
    
    if (filtered.length === 0) {
        grid.innerHTML = '<div class="col-span-full text-center py-12 text-gray-400">No movies found</div>';
        return;
    }
    
    // Render movie cards
    filtered.forEach(movie => {
        const card = document.createElement('article');
        card.className = 'group bg-white/2 rounded overflow-hidden transform hover:scale-[1.02] transition cursor-pointer';
        card.onclick = () => showMovieDetails(movie);
        
        card.innerHTML = `
            <div class='relative'>
                <img src='${movie.poster}' alt='${movie.title}' class='w-full h-56 object-cover' onerror="this.src='https://via.placeholder.com/300x450/1a1a1a/ffffff?text=No+Image'">
                <div class='absolute left-2 top-2 bg-black/60 px-2 py-1 text-xs rounded'>${movie.year}</div>
            </div>
            <div class='p-2'>
                <h4 class='text-sm font-semibold truncate'>${movie.title}</h4>
                <div class='flex items-center justify-between text-xs text-gray-300 mt-1'>
                    <span>${movie.genre}</span>
                    <span class='bg-white/6 px-2 py-0.5 rounded'>⭐ ${movie.rating}</span>
                </div>
            </div>
        `;
        
        grid.appendChild(card);
    });
}

// Show movie details with video player
async function showMovieDetails(movie) {
    try {
        const response = await fetch(`${API_BASE}/movies/${movie.id}/videos`);
        const data = await response.json();
        
        let videoHtml = '';
        if (data.results && data.results.length > 0) {
            const trailer = data.results.find(v => v.type === 'Trailer' && v.site === 'YouTube') || data.results[0];
            if (trailer && trailer.key) {
                videoHtml = `
                    <div style="margin: 20px 0;">
                        <iframe width="100%" height="400" 
                            src="https://www.youtube.com/embed/${trailer.key}" 
                            frameborder="0" 
                            allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture" 
                            allowfullscreen>
                        </iframe>
                    </div>
                `;
            }
        }
        
        // Create modal
        const modal = document.createElement('div');
        modal.style.cssText = 'position:fixed;top:0;left:0;right:0;bottom:0;background:rgba(0,0,0,0.95);z-index:9999;overflow:auto;padding:20px;';
        modal.innerHTML = `
            <div style="max-width:900px;margin:auto;background:#1a1a1a;border-radius:12px;padding:30px;position:relative;">
                <button onclick="this.parentElement.parentElement.remove()" 
                    style="position:absolute;top:10px;right:10px;background:#ff4444;color:white;border:none;padding:10px 15px;border-radius:5px;cursor:pointer;font-size:18px;">
                    ✕
                </button>
                <h2 style="color:#fbbf24;margin-bottom:10px;">${movie.title}</h2>
                <p style="color:#9ca3af;margin-bottom:20px;">${movie.genre} • ${movie.year} • ⭐ ${movie.rating}/10</p>
                ${videoHtml}
                <p style="line-height:1.6;color:#d1d5db;">${movie.description}</p>
                <div style="margin-top:20px;">
                    <button onclick="window.open('https://www.themoviedb.org/movie/${movie.id}', '_blank')" 
                        style="background:#fbbf24;color:#000;border:none;padding:12px 24px;border-radius:6px;cursor:pointer;font-weight:bold;margin-right:10px;">
                        View on TMDb
                    </button>
                    ${videoHtml ? '<button onclick="this.parentElement.parentElement.parentElement.remove()" style="background:#374151;color:white;border:none;padding:12px 24px;border-radius:6px;cursor:pointer;">Close</button>' : ''}
                </div>
            </div>
        `;
        document.body.appendChild(modal);
    } catch (error) {
        console.error('Error loading movie details:', error);
        alert(`${movie.title}\n\n${movie.description || 'No description available.'}\n\nRating: ${movie.rating}/10\nYear: ${movie.year}\nGenre: ${movie.genre}`);
    }
}

// Filter functions
function filterByCategory(category) {
    currentFilter.category = category;
    currentFilter.search = ''; // Clear search
    document.getElementById('search').value = ''; // Clear search input
    const categoryMap = {
        'all': 'trending',
        'movie': 'popular',
        'tv': 'popular',
        'top': 'top_rated'
    };
    fetchMovies(1, categoryMap[category] || 'trending');
}

function filterByGenre(genre) {
    currentFilter.genre = genre;
    currentFilter.search = ''; // Clear search
    document.getElementById('search').value = ''; // Clear search input
    renderMovies();
}

function filterByYear(year) {
    currentFilter.year = year;
    currentFilter.search = ''; // Clear search
    document.getElementById('search').value = ''; // Clear search input
    renderMovies();
}

function changePage(direction) {
    currentPage += direction;
    if (currentPage < 1) currentPage = 1;
    if (currentPage > totalPages) currentPage = totalPages;
    fetchMovies(currentPage, currentFilter.category);
}

function updatePagination() {
    document.getElementById('current-page').textContent = currentPage;
}

// Search functionality
let searchTimeout;
document.getElementById('search').addEventListener('input', (e) => {
    const query = e.target.value.trim();
    
    // Clear previous timeout
    if (searchTimeout) {
        clearTimeout(searchTimeout);
    }
    
    // Debounce search
    searchTimeout = setTimeout(async () => {
        if (query.length === 0) {
            // If search is empty, go back to trending
            fetchMovies(1, 'trending');
        } else if (query.length >= 2) {
            // Search TMDb database
            try {
                const loading = document.getElementById('loading');
                const grid = document.getElementById('movies-grid');
                
                loading.style.display = 'block';
                grid.innerHTML = '';
                
                const response = await fetch(`${API_BASE}/search?q=${encodeURIComponent(query)}`);
                const data = await response.json();
                
                allMovies = data.movies || [];
                currentFilter.search = query;
                
                loading.style.display = 'none';
                renderMovies();
                
                // Update section title
                document.getElementById('section-title').textContent = 
                    allMovies.length > 0 
                        ? `Search results for "${query}" (${allMovies.length} found)` 
                        : `No results for "${query}"`;
            } catch (error) {
                console.error('Search error:', error);
                document.getElementById('loading').textContent = 'Search failed. Please try again.';
            }
        }
    }, 500); // Wait 500ms after user stops typing
});

// Initialize
fetchMovies();
