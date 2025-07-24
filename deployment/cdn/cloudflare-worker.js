/**
 * CDN Configuration for Marco 2.0
 * CloudFlare Workers script for edge optimization
 */

export default {
  async fetch(request, env, ctx) {
    const url = new URL(request.url);
    const cache = caches.default;
    
    // Security headers
    const securityHeaders = {
      'X-Frame-Options': 'DENY',
      'X-Content-Type-Options': 'nosniff',
      'X-XSS-Protection': '1; mode=block',
      'Referrer-Policy': 'strict-origin-when-cross-origin',
      'Permissions-Policy': 'camera=(), microphone=(), geolocation=()',
      'Content-Security-Policy': "default-src 'self'; script-src 'self' 'unsafe-inline' 'wasm-unsafe-eval'; worker-src 'self' blob:; connect-src 'self' wss: ws: https:; media-src 'self' blob:; img-src 'self' data: blob:; style-src 'self' 'unsafe-inline'; font-src 'self' data:; object-src 'none'; base-uri 'self'; form-action 'self'"
    };

    // Handle WASM files with special MIME type and headers
    if (url.pathname.endsWith('.wasm')) {
      const cacheKey = new Request(url.toString(), request);
      let response = await cache.match(cacheKey);
      
      if (!response) {
        response = await fetch(request);
        
        if (response.ok) {
          // Clone and modify headers
          const modifiedResponse = new Response(response.body, {
            status: response.status,
            statusText: response.statusText,
            headers: {
              ...Object.fromEntries(response.headers),
              'Content-Type': 'application/wasm',
              'Cross-Origin-Embedder-Policy': 'require-corp',
              'Cross-Origin-Opener-Policy': 'same-origin',
              'Cache-Control': 'public, max-age=31536000, immutable',
              ...securityHeaders
            }
          });
          
          // Cache for 1 year
          ctx.waitUntil(cache.put(cacheKey, modifiedResponse.clone()));
          return modifiedResponse;
        }
      }
      
      return response;
    }

    // Handle JavaScript and CSS with compression and caching
    if (url.pathname.endsWith('.js') || url.pathname.endsWith('.css')) {
      const cacheKey = new Request(url.toString(), request);
      let response = await cache.match(cacheKey);
      
      if (!response) {
        response = await fetch(request);
        
        if (response.ok) {
          const modifiedResponse = new Response(response.body, {
            status: response.status,
            statusText: response.statusText,
            headers: {
              ...Object.fromEntries(response.headers),
              'Cache-Control': 'public, max-age=31536000, immutable',
              'Content-Encoding': 'br', // Brotli compression
              ...securityHeaders
            }
          });
          
          ctx.waitUntil(cache.put(cacheKey, modifiedResponse.clone()));
          return modifiedResponse;
        }
      }
      
      return response;
    }

    // Handle Service Worker with no caching
    if (url.pathname === '/sw.js') {
      const response = await fetch(request);
      
      if (response.ok) {
        return new Response(response.body, {
          status: response.status,
          statusText: response.statusText,
          headers: {
            ...Object.fromEntries(response.headers),
            'Cache-Control': 'no-cache, no-store, must-revalidate',
            'Pragma': 'no-cache',
            ...securityHeaders
          }
        });
      }
      
      return response;
    }

    // Handle API requests with rate limiting
    if (url.pathname.startsWith('/api/')) {
      const clientIP = request.headers.get('CF-Connecting-IP');
      const rateLimitKey = `rate-limit:${clientIP}`;
      
      // Simple rate limiting (100 requests per minute)
      const currentCount = await env.RATE_LIMIT_KV.get(rateLimitKey);
      const count = currentCount ? parseInt(currentCount) : 0;
      
      if (count > 100) {
        return new Response('Rate limit exceeded', { 
          status: 429,
          headers: securityHeaders
        });
      }
      
      // Increment counter
      await env.RATE_LIMIT_KV.put(rateLimitKey, (count + 1).toString(), { expirationTtl: 60 });
      
      const response = await fetch(request);
      
      // Add security headers to API responses
      return new Response(response.body, {
        status: response.status,
        statusText: response.statusText,
        headers: {
          ...Object.fromEntries(response.headers),
          ...securityHeaders,
          'X-API-Version': '4.3.0'
        }
      });
    }

    // Handle WebSocket upgrade requests
    if (request.headers.get('Upgrade') === 'websocket') {
      // Pass through WebSocket requests to origin
      return fetch(request);
    }

    // Default response with security headers
    const response = await fetch(request);
    
    if (response.ok) {
      return new Response(response.body, {
        status: response.status,
        statusText: response.statusText,
        headers: {
          ...Object.fromEntries(response.headers),
          ...securityHeaders
        }
      });
    }
    
    return response;
  }
};
