pub fn get_adblocker_script(enabled: bool, whitelist: &Option<Vec<String>>, blacklist: &Option<Vec<String>>) -> String {
    if enabled {
        let whitelist_js = match whitelist {
            Some(domains) => {
                let domains_array = domains.iter()
                    .map(|d| format!("'{}'", d))
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("[{}]", domains_array)
            }
            None => "[]".to_string(),
        };
        
        let blacklist_js = match blacklist {
            Some(domains) => {
                let domains_array = domains.iter()
                    .map(|d| format!("'{}'", d))
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("[{}]", domains_array)
            }
            None => "[]".to_string(),
        };
        
        format!("
        // Enhanced adblocker implementation
        (function() {{
            // Whitelist and blacklist from config
            const whitelistedDomains = {};
            const blacklistedDomains = {};
            
            const blockedSelectors = [
                // Common ad containers
                '.ad', '.ads', '.advertisement', '.ad-container', '.ad-wrapper',
                '.google-ads', '.adsense', '.ad-banner', '.ad-sidebar', '.popup-ad',
                '.sponsored', '.promotion', '.promo', '.commercial', '.marketing',
                
                // ID and class patterns
                '[id*=\"ad\"]', '[class*=\"ad\"]', '[id*=\"ads\"]', '[class*=\"ads\"]',
                '[id*=\"sponsor\"]', '[class*=\"sponsor\"]', '[id*=\"promo\"]', '[class*=\"promo\"]',
                
                // Iframe and embed patterns
                'iframe[src*=\"ads\"]', 'iframe[src*=\"doubleclick\"]', 'iframe[src*=\"googleads\"]',
                'iframe[src*=\"amazon-adsystem\"]', 'iframe[src*=\"facebook\"]', 'iframe[src*=\"twitter\"]',
                'embed[src*=\"ads\"]', 'object[type*=\"ad\"]',
                
                // Data attributes
                '[data-ad-client]', '[data-ad-slot]', '[data-ad-format]', '[data-ads]',
                '[data-google-ad]', '[data-amazon-ad]', '[data-fb-ad]',
                
                // Common ad networks
                '.google-auto-placed', '.adsbygoogle', '.google-adsense',
                '.fb-ad', '.facebook-ad', '.twitter-ad', '.instagram-ad',
                
                // Video ads
                '.video-ad', '.preroll-ad', '.midroll-ad', '.postroll-ad',
                '.youtube-ad', '.vimeo-ad', '.player-ad',
                
                // Popups and overlays
                '.popup', '.modal-ad', '.overlay-ad', '.interstitial',
                '.lightbox-ad', '.dialog-ad', '.toast-ad',
                
                // Native advertising
                '.native-ad', '.recommended-ad', '.sponsored-content',
                '.partner-content', '.ad-feature', '.paid-content'
            ];
            
            const blockedUrls = [
                // Only block obvious ad/tracking domains
                'doubleclick.net', 'googleadservices.com', 'googlesyndication.com',
                'googleads.g.doubleclick.net', 'pagead2.googlesyndication.com',
                'tpc.googlesyndication.com', 'amazon-adsystem.com', 'taboola.com',
                'outbrain.com', 'adnxs.com', 'criteo.com', 'adsafeprotected.com',
                
                // Specific tracking endpoints only
                '/analytics.js', '/gtm.js', '/gtag.js', '/fbevents.js',
                '/adsct', '/collect', '/j/collect', '/track', '/v1/p'
            ];
            
            function isWhitelisted() {{
                return whitelistedDomains.some(domain => 
                    window.location.hostname === domain || 
                    window.location.hostname.endsWith('.' + domain)
                );
            }}
            
            function isBlacklisted() {{
                return blacklistedDomains.some(domain => 
                    window.location.hostname === domain || 
                    window.location.hostname.endsWith('.' + domain)
                );
            }}
            
            function blockAds() {{
                // Skip blocking on whitelisted sites
                if (isWhitelisted()) {{
                    console.log('Adblocker disabled on whitelisted domain:', window.location.hostname);
                    return;
                }}
                
                // Force block on blacklisted sites (more aggressive)
                const isAggressive = isBlacklisted();
                
                // Block by selectors
                blockedSelectors.forEach(selector => {{
                    try {{
                        const elements = document.querySelectorAll(selector);
                        elements.forEach(el => {{
                            el.style.display = 'none';
                            el.style.visibility = 'hidden';
                            el.style.opacity = '0';
                            el.style.pointerEvents = 'none';
                            el.setAttribute('data-adblocked', 'true');
                            // Remove after a short delay to avoid layout shifts
                            setTimeout(() => el.remove(), 100);
                        }});
                    }} catch(e) {{}}
                }});
                
                // Block images and scripts from ad domains
                document.querySelectorAll('img, script, iframe, embed').forEach(el => {{
                    const src = el.src || el.getAttribute('data-src');
                    if (src && blockedUrls.some(url => src.includes(url))) {{
                        el.style.display = 'none';
                        el.setAttribute('data-adblocked', 'true');
                        el.remove();
                    }}
                }});
                
                // Block network requests (intercept fetch/XHR) - more conservative
                const originalFetch = window.fetch;
                window.fetch = function(...args) {{
                    const url = args[0];
                    if (typeof url === 'string' && blockedUrls.some(blocked => url.includes(blocked))) {{
                        // Only block if it's clearly an ad/tracking request and not whitelisted
                        if (!isWhitelisted() && 
                            (url.includes('doubleclick') || url.includes('adsystem') || 
                             url.includes('taboola') || url.includes('outbrain') ||
                             url.includes('/ads') || url.includes('/tracking'))) {{
                            console.log('Blocked fetch request:', url);
                            return Promise.reject(new Error('Blocked by adblocker'));
                        }}
                    }}
                    return originalFetch.apply(this, args);
                }};
                
                // Block XMLHttpRequest - more conservative
                const originalXHR = window.XMLHttpRequest.prototype.open;
                window.XMLHttpRequest.prototype.open = function(method, url, ...args) {{
                    if (typeof url === 'string' && blockedUrls.some(blocked => url.includes(blocked))) {{
                        // Only block if it's clearly an ad/tracking request and not whitelisted
                        if (!isWhitelisted() && 
                            (url.includes('doubleclick') || url.includes('adsystem') || 
                             url.includes('taboola') || url.includes('outbrain') ||
                             url.includes('/ads') || url.includes('/tracking'))) {{
                            console.log('Blocked XHR request:', url);
                            throw new Error('Blocked by adblocker');
                        }}
                    }}
                    return originalXHR.call(this, method, url, ...args);
                }};
            }}
            
            // Block ads on initial load
            if (document.readyState === 'loading') {{
                document.addEventListener('DOMContentLoaded', blockAds);
            }} else {{
                blockAds();
            }}
            
            // Block ads on dynamic content changes
            const observer = new MutationObserver((mutations) => {{
                // Skip on whitelisted sites
                if (isWhitelisted()) return;
                
                let shouldBlock = false;
                mutations.forEach(mutation => {{
                    if (mutation.type === 'childList' && mutation.addedNodes.length > 0) {{
                        shouldBlock = true;
                    }}
                }});
                if (shouldBlock) {{
                    setTimeout(blockAds, 50); // Small delay to let content render
                }}
            }});
            
            observer.observe(document.body || document.documentElement, {{
                childList: true,
                subtree: true,
                attributes: true,
                attributeFilter: ['src', 'href', 'data-src']
            }});
            
            // Periodic cleanup (only on non-whitelisted sites)
            if (!isWhitelisted()) {{
                setInterval(blockAds, 5000);
            }}
            
            // Console log for debugging
            console.log('Enhanced adblocker initialized' + 
                (isWhitelisted() ? ' (whitelisted mode)' : '') + 
                (isBlacklisted() ? ' (blacklisted mode - aggressive)' : ''));
        }})();
        ", whitelist_js, blacklist_js)
    } else {
        String::new()
    }
}
