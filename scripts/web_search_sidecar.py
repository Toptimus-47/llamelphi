from fastapi import FastAPI, Query
from playwright.async_api import async_playwright
import uvicorn
import asyncio

app = FastAPI(title="MAGI Web Search Sidecar")

async def scrape_duckduckgo(query: str):
    async with async_playwright() as p:
        # Launch headless browser
        browser = await p.chromium.launch(headless=True)
        context = await browser.new_context(
            user_agent="Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36"
        )
        page = await context.new_page()
        
        # Go to DuckDuckGo (less bot detection than Google)
        search_url = f"https://duckduckgo.com/html/?q={query}"
        await page.goto(search_url)
        
        # Extract results
        results = []
        # DuckDuckGo HTML version uses .result__body
        elements = await page.query_selector_all(".result__body")
        
        for el in elements[:3]: # Limit to top 3
            title_el = await el.query_selector(".result__title")
            snippet_el = await el.query_selector(".result__snippet")
            link_el = await el.query_selector(".result__url")
            
            if title_el and snippet_el:
                title = await title_el.inner_text()
                snippet = await snippet_el.inner_text()
                link = await link_el.inner_text() if link_el else "N/A"
                results.append({
                    "title": title.strip(),
                    "url": link.strip(),
                    "content": snippet.strip()
                })
        
        await browser.close()
        return results

@app.get("/search")
async def search(q: str = Query(..., min_length=1)):
    print(f"[SIDECAR] Searching for: {q}")
    try:
        results = await scrape_duckduckgo(q)
        return {"results": results}
    except Exception as e:
        print(f"[SIDECAR] Error: {e}")
        return {"results": [], "error": str(e)}

if __name__ == "__main__":
    print("====================================================")
    print("   MAGI WEB SEARCH SIDECAR (PLAYWRIGHT) STARTING    ")
    print("   Listening on http://127.0.0.1:8001               ")
    print("====================================================")
    uvicorn.run(app, host="127.0.0.1", port=8001)
