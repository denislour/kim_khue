// Progressive enhancement: books and native <details> remain readable without JS.
const search = document.querySelector('#story-search');
const cards = [...document.querySelectorAll('.story')];
const filters = [...document.querySelectorAll('input[name="genre"]')];
const count = document.querySelector('#results-count');
const noResults = document.querySelector('#no-results');
const params = new URLSearchParams(location.search);
let genre = filters.some(button => button.dataset.genre === params.get('genre')) ? params.get('genre') : 'all';
const normalize = text => text.toLocaleLowerCase().normalize('NFD').replace(/\p{M}/gu, '').replace(/đ/g, 'd');

function updateLibrary(syncUrl = true) {
  const query = normalize(search.value.trim());
  let visible = 0;
  for (const card of cards) {
    const text = `${card.querySelector('.story-title').textContent} ${card.querySelector('.story-summary').textContent}`;
    const matches = (genre === 'all' || card.dataset.genre === genre) && normalize(text).includes(query);
    card.hidden = !matches;
    if (matches) visible++;
  }
  for (const option of filters) option.checked = option.dataset.genre === genre;
  count.textContent = `${new Intl.NumberFormat(document.documentElement.lang).format(visible)} ${visible === 1 ? count.dataset.one : count.dataset.label}`;
  noResults.hidden = visible !== 0;
  if (syncUrl) {
    const url = new URL(location.href);
    search.value.trim() ? url.searchParams.set('q', search.value.trim()) : url.searchParams.delete('q');
    genre !== 'all' ? url.searchParams.set('genre', genre) : url.searchParams.delete('genre');
    history.replaceState(null, '', url);
  }
}
if (search) {
  document.querySelector('#search-form').hidden = false;
  document.querySelector('#library-tools').hidden = false;
  search.value = params.get('q') || '';
  search.addEventListener('input', () => updateLibrary());
  document.querySelector('#search-form').addEventListener('submit', event => event.preventDefault());
  filters.forEach(option => option.addEventListener('change', () => { genre = option.dataset.genre; updateLibrary(); }));
  document.querySelector('#clear-search').addEventListener('click', () => { search.value = ''; genre = 'all'; updateLibrary(); search.focus(); });
  updateLibrary(false);
}

const reader = document.querySelector('#reader');
let readingTrigger;
function closeReader() { reader.close(); }
if (reader && typeof reader.showModal === 'function') {
  for (const card of cards) {
    const trigger = card.querySelector('summary');
    trigger.addEventListener('click', event => {
      event.preventDefault();
      readingTrigger = trigger;
      document.querySelector('#reader-title').textContent = card.querySelector('.story-title').textContent;
      document.querySelector('#reader-chapter').textContent = card.querySelector('.excerpt h4').textContent;
      const content = document.querySelector('#reader-content');
      content.replaceChildren(...[...card.querySelectorAll('.excerpt p')].map(p => p.cloneNode(true)));
      reader.showModal();
      reader.querySelector('.reader-panel').scrollTop = 0;
      document.body.classList.add('reading');
      document.querySelector('#reader-close').focus();
    });
  }
  const headerReading = document.querySelector('.header-reading');
  headerReading?.addEventListener('click', event => {
    const firstPassage = cards[0]?.querySelector('summary');
    if (!firstPassage) return;
    event.preventDefault();
    firstPassage.click();
    readingTrigger = headerReading;
  });
  document.querySelector('#reader-close').addEventListener('click', closeReader);
  reader.addEventListener('click', event => {
    if (event.target === reader) closeReader();
  });
  reader.addEventListener('close', () => { document.body.classList.remove('reading'); readingTrigger?.focus(); });
  document.querySelector('#reader-size').addEventListener('input', event => reader.style.setProperty('--reader-size', `${event.target.value}px`));
  document.querySelector('#reader-theme').addEventListener('change', event => {
    reader.classList.toggle('dark', event.currentTarget.checked);
  });
}
