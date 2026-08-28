import { createHash } from "node:crypto";
import { readdir, readFile, writeFile } from "node:fs/promises";
import { join, relative, sep } from "node:path";

const root = new URL("../dist/site/", import.meta.url);

async function filesUnder(directory) {
  const entries = await readdir(directory, { withFileTypes: true });
  const files = [];
  for (const entry of entries) {
    const path = join(directory.pathname, entry.name);
    if (entry.isDirectory()) files.push(...await filesUnder(new URL(`file://${path}/`)));
    else if (entry.name !== "sw.js" && entry.name !== "staticwebapp.config.json" && !entry.name.endsWith(".map")) files.push(path);
  }
  return files;
}

const files = (await filesUnder(root)).sort();
const hash = createHash("sha256");
const urls = [];
for (const file of files) {
  const content = await readFile(file);
  hash.update(content);
  let url = `/${relative(root.pathname, file).split(sep).join("/")}`;
  if (url.endsWith("/index.html")) url = url.slice(0, -10) || "/";
  urls.push(url);
}
const version = hash.digest("hex").slice(0, 12);
const source = `const CACHE = "sentinel-${version}";
const PRECACHE = ${JSON.stringify(urls)};
self.addEventListener("install", event => event.waitUntil(caches.open(CACHE).then(cache => cache.addAll(PRECACHE)).then(() => self.skipWaiting())));
self.addEventListener("activate", event => event.waitUntil(caches.keys().then(keys => Promise.all(keys.filter(key => key.startsWith("sentinel-") && key !== CACHE).map(key => caches.delete(key)))).then(() => self.clients.claim())));
self.addEventListener("fetch", event => {
  const url = new URL(event.request.url);
  if (url.searchParams.has("online-check") || event.request.method !== "GET" || url.origin !== self.location.origin) return;
  event.respondWith(caches.match(event.request).then(cached => cached || fetch(event.request).then(response => {
    if (response.ok) caches.open(CACHE).then(cache => cache.put(event.request, response.clone()));
    return response;
  })));
});
`;
await writeFile(new URL("sw.js", root), source);
console.log(`service worker: ${urls.length} files, cache ${version}`);
