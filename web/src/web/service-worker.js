/**
 * Marco 2.0 Service Worker
 * 
 * Provides offline functionality, caching, and background sync for the web application
 */

const CACHE_NAME = 'marco-2-v0.1.0';
const STATIC_CACHE_NAME = 'marco-2-static-v0.1.0';
const DYNAMIC_CACHE_NAME = 'marco-2-dynamic-v0.1.0';

// Cache strategies
const CACHE_STRATEGIES = {
  CACHE_FIRST: 'cache-first',
  NETWORK_FIRST: 'network-first',
  STALE_WHILE_REVALIDATE: 'stale-while-revalidate',
  NETWORK_ONLY: 'network-only',
  CACHE_ONLY: 'cache-only',
};

// Files to cache for offline functionality
const STATIC_ASSETS = [
  '/',
  '/index.html',
  '/manifest.json',
  '/pkg/marco_2_web.js',
  '/pkg/marco_2_web_bg.wasm',
  // CSS and JS bundles will be added dynamically
];

// Dynamic content patterns
const DYNAMIC_PATTERNS = {
  API_CALLS: /^\/api\//,
  USER_PROJECTS: /^\/projects\//,
  TEMPLATES: /^\/templates\//,
  ASSETS: /\.(png|jpg|jpeg|gif|svg|webp|ico)$/,
};

// Background sync tags
const SYNC_TAGS = {
  PROJECT_SAVE: 'project-save',
  SETTINGS_SYNC: 'settings-sync',
  ANALYTICS: 'analytics',
};

/**
 * Service Worker Installation
 */
self.addEventListener('install', (event) => {
  console.log('Marco 2.0 Service Worker: Installing...');
  
  event.waitUntil(
    (async () => {
      try {
        // Create caches
        const staticCache = await caches.open(STATIC_CACHE_NAME);
        const dynamicCache = await caches.open(DYNAMIC_CACHE_NAME);
        
        // Cache static assets
        await staticCache.addAll(STATIC_ASSETS);
        
        // Get and cache webpack assets
        const assetManifest = await getAssetManifest();
        if (assetManifest) {
          const webpackAssets = Object.values(assetManifest);
          await staticCache.addAll(webpackAssets);
        }
        
        console.log('Marco 2.0 Service Worker: Static assets cached');
        
        // Skip waiting to activate immediately
        self.skipWaiting();
        
      } catch (error) {
        console.error('Marco 2.0 Service Worker: Installation failed', error);
      }
    })()
  );
});

/**
 * Service Worker Activation
 */
self.addEventListener('activate', (event) => {
  console.log('Marco 2.0 Service Worker: Activating...');
  
  event.waitUntil(
    (async () => {
      try {
        // Clean up old caches
        const cacheNames = await caches.keys();
        await Promise.all(
          cacheNames
            .filter(name => name.startsWith('marco-2-') && 
                           name !== STATIC_CACHE_NAME && 
                           name !== DYNAMIC_CACHE_NAME)
            .map(name => caches.delete(name))
        );
        
        // Take control of all pages
        await self.clients.claim();
        
        console.log('Marco 2.0 Service Worker: Activated successfully');
        
        // Notify clients of activation
        const clients = await self.clients.matchAll();
        clients.forEach(client => {
          client.postMessage({
            type: 'SW_ACTIVATED',
            version: CACHE_NAME,
          });
        });
        
      } catch (error) {
        console.error('Marco 2.0 Service Worker: Activation failed', error);
      }
    })()
  );
});

/**
 * Fetch Event Handler - Main caching logic
 */
self.addEventListener('fetch', (event) => {
  const { request } = event;
  const url = new URL(request.url);
  
  // Skip non-HTTP requests
  if (!url.protocol.startsWith('http')) {
    return;
  }
  
  // Determine cache strategy based on request
  const strategy = getCacheStrategy(request);
  
  event.respondWith(
    handleRequest(request, strategy)
  );
});

/**
 * Background Sync Handler
 */
self.addEventListener('sync', (event) => {
  console.log('Marco 2.0 Service Worker: Background sync triggered', event.tag);
  
  switch (event.tag) {
    case SYNC_TAGS.PROJECT_SAVE:
      event.waitUntil(syncProjectSave());
      break;
      
    case SYNC_TAGS.SETTINGS_SYNC:
      event.waitUntil(syncSettings());
      break;
      
    case SYNC_TAGS.ANALYTICS:
      event.waitUntil(syncAnalytics());
      break;
      
    default:
      console.warn('Unknown sync tag:', event.tag);
  }
});

/**
 * Push Message Handler
 */
self.addEventListener('push', (event) => {
  console.log('Marco 2.0 Service Worker: Push message received');
  
  const options = {
    body: 'Marco 2.0 has been updated with new features!',
    icon: '/assets/icons/icon-192x192.png',
    badge: '/assets/icons/badge.png',
    vibrate: [100, 50, 100],
    data: {
      dateOfArrival: Date.now(),
      primaryKey: 1,
    },
    actions: [
      {
        action: 'explore',
        title: 'Explore New Features',
        icon: '/assets/icons/explore.png',
      },
      {
        action: 'close',
        title: 'Close',
        icon: '/assets/icons/close.png',
      },
    ],
  };
  
  if (event.data) {
    const payload = event.data.json();
    options.body = payload.body || options.body;
    options.title = payload.title || 'Marco 2.0';
  }
  
  event.waitUntil(
    self.registration.showNotification('Marco 2.0', options)
  );
});

/**
 * Notification Click Handler
 */
self.addEventListener('notificationclick', (event) => {
  console.log('Marco 2.0 Service Worker: Notification clicked', event.action);
  
  event.notification.close();
  
  if (event.action === 'explore') {
    event.waitUntil(
      clients.openWindow('/?utm_source=notification&utm_medium=push')
    );
  }
});

/**
 * Message Handler - Communication with main app
 */
self.addEventListener('message', (event) => {
  const { type, data } = event.data;
  
  switch (type) {
    case 'SKIP_WAITING':
      self.skipWaiting();
      break;
      
    case 'GET_VERSION':
      event.ports[0].postMessage({ version: CACHE_NAME });
      break;
      
    case 'CLEAR_CACHE':
      clearAllCaches().then(() => {
        event.ports[0].postMessage({ success: true });
      });
      break;
      
    case 'CACHE_PROJECT':
      cacheProject(data).then(() => {
        event.ports[0].postMessage({ success: true });
      });
      break;
      
    default:
      console.warn('Unknown message type:', type);
  }
});

/**
 * Cache Strategy Determination
 */
function getCacheStrategy(request) {
  const url = new URL(request.url);
  const pathname = url.pathname;
  
  // WASM files - Cache first (they rarely change)
  if (pathname.endsWith('.wasm')) {
    return CACHE_STRATEGIES.CACHE_FIRST;
  }
  
  // Static assets - Cache first
  if (pathname.match(/\.(js|css|png|jpg|jpeg|gif|svg|webp|ico|woff2?|ttf)$/)) {
    return CACHE_STRATEGIES.CACHE_FIRST;
  }
  
  // HTML files - Stale while revalidate
  if (pathname.endsWith('.html') || pathname === '/') {
    return CACHE_STRATEGIES.STALE_WHILE_REVALIDATE;
  }
  
  // API calls - Network first
  if (DYNAMIC_PATTERNS.API_CALLS.test(pathname)) {
    return CACHE_STRATEGIES.NETWORK_FIRST;
  }
  
  // User projects - Network first with fallback
  if (DYNAMIC_PATTERNS.USER_PROJECTS.test(pathname)) {
    return CACHE_STRATEGIES.NETWORK_FIRST;
  }
  
  // Default - Network first
  return CACHE_STRATEGIES.NETWORK_FIRST;
}

/**
 * Request Handler with Caching Logic
 */
async function handleRequest(request, strategy) {
  const cache = await caches.open(DYNAMIC_CACHE_NAME);
  
  switch (strategy) {
    case CACHE_STRATEGIES.CACHE_FIRST:
      return await cacheFirst(request, cache);
      
    case CACHE_STRATEGIES.NETWORK_FIRST:
      return await networkFirst(request, cache);
      
    case CACHE_STRATEGIES.STALE_WHILE_REVALIDATE:
      return await staleWhileRevalidate(request, cache);
      
    case CACHE_STRATEGIES.NETWORK_ONLY:
      return await fetch(request);
      
    case CACHE_STRATEGIES.CACHE_ONLY:
      return await cache.match(request);
      
    default:
      return await networkFirst(request, cache);
  }
}

/**
 * Cache First Strategy
 */
async function cacheFirst(request, cache) {
  const cachedResponse = await cache.match(request);
  
  if (cachedResponse) {
    return cachedResponse;
  }
  
  try {
    const networkResponse = await fetch(request);
    
    if (networkResponse.ok) {
      cache.put(request, networkResponse.clone());
    }
    
    return networkResponse;
  } catch (error) {
    console.error('Network request failed:', error);
    return new Response('Offline', { status: 503 });
  }
}

/**
 * Network First Strategy
 */
async function networkFirst(request, cache) {
  try {
    const networkResponse = await fetch(request);
    
    if (networkResponse.ok) {
      cache.put(request, networkResponse.clone());
    }
    
    return networkResponse;
  } catch (error) {
    const cachedResponse = await cache.match(request);
    
    if (cachedResponse) {
      return cachedResponse;
    }
    
    return new Response('Offline', { status: 503 });
  }
}

/**
 * Stale While Revalidate Strategy
 */
async function staleWhileRevalidate(request, cache) {
  const cachedResponse = await cache.match(request);
  
  const networkResponsePromise = fetch(request).then(networkResponse => {
    if (networkResponse.ok) {
      cache.put(request, networkResponse.clone());
    }
    return networkResponse;
  }).catch(() => null);
  
  return cachedResponse || await networkResponsePromise;
}

/**
 * Background Sync Functions
 */
async function syncProjectSave() {
  try {
    const pendingSaves = await getStoredData('pendingProjectSaves');
    
    for (const save of pendingSaves || []) {
      await fetch('/api/projects/save', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(save),
      });
    }
    
    await clearStoredData('pendingProjectSaves');
    console.log('Project saves synced successfully');
  } catch (error) {
    console.error('Project sync failed:', error);
  }
}

async function syncSettings() {
  try {
    const pendingSettings = await getStoredData('pendingSettingsSync');
    
    if (pendingSettings) {
      await fetch('/api/settings/sync', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(pendingSettings),
      });
      
      await clearStoredData('pendingSettingsSync');
      console.log('Settings synced successfully');
    }
  } catch (error) {
    console.error('Settings sync failed:', error);
  }
}

async function syncAnalytics() {
  try {
    const pendingAnalytics = await getStoredData('pendingAnalytics');
    
    for (const event of pendingAnalytics || []) {
      await fetch('/api/analytics/event', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(event),
      });
    }
    
    await clearStoredData('pendingAnalytics');
    console.log('Analytics synced successfully');
  } catch (error) {
    console.error('Analytics sync failed:', error);
  }
}

/**
 * Utility Functions
 */
async function getAssetManifest() {
  try {
    const response = await fetch('/asset-manifest.json');
    return await response.json();
  } catch (error) {
    console.warn('Could not load asset manifest:', error);
    return null;
  }
}

async function clearAllCaches() {
  const cacheNames = await caches.keys();
  await Promise.all(
    cacheNames
      .filter(name => name.startsWith('marco-2-'))
      .map(name => caches.delete(name))
  );
}

async function cacheProject(projectData) {
  const cache = await caches.open(DYNAMIC_CACHE_NAME);
  const response = new Response(JSON.stringify(projectData), {
    headers: { 'Content-Type': 'application/json' },
  });
  
  await cache.put(`/projects/${projectData.id}`, response);
}

async function getStoredData(key) {
  try {
    const db = await openDB();
    const transaction = db.transaction(['marco2'], 'readonly');
    const store = transaction.objectStore('marco2');
    return await store.get(key);
  } catch (error) {
    console.error('Failed to get stored data:', error);
    return null;
  }
}

async function clearStoredData(key) {
  try {
    const db = await openDB();
    const transaction = db.transaction(['marco2'], 'readwrite');
    const store = transaction.objectStore('marco2');
    await store.delete(key);
  } catch (error) {
    console.error('Failed to clear stored data:', error);
  }
}

async function openDB() {
  return new Promise((resolve, reject) => {
    const request = indexedDB.open('marco2-db', 1);
    
    request.onerror = () => reject(request.error);
    request.onsuccess = () => resolve(request.result);
    
    request.onupgradeneeded = (event) => {
      const db = event.target.result;
      if (!db.objectStoreNames.contains('marco2')) {
        db.createObjectStore('marco2');
      }
    };
  });
}

console.log('Marco 2.0 Service Worker loaded successfully');
