
/**
 * @roxi/routify 2.18.18
 * File generated Wed Oct 08 2025 09:30:58 GMT+0200 (Central European Summer Time)
 */

export const __version = "2.18.18"
export const __timestamp = "2025-10-08T07:30:58.430Z"

//buildRoutes
import { buildClientTree } from "@roxi/routify/runtime/buildRoutes.js"

//imports


//options
export const options = {}

//tree
export const _tree = {
  "name": "_layout",
  "filepath": "/_layout.svelte",
  "root": true,
  "ownMeta": {},
  "absolutePath": "/Users/safokofi/Desktop/internship/20251007_kofi_task_006_intuivo/20251007_task_kofi _006/dance_event/berlin-dancemode-clone/frontend/src/pages/_layout.svelte",
  "children": [
    {
      "isFile": true,
      "isDir": false,
      "file": "events.svelte",
      "filepath": "/events.svelte",
      "name": "events",
      "ext": "svelte",
      "badExt": false,
      "absolutePath": "/Users/safokofi/Desktop/internship/20251007_kofi_task_006_intuivo/20251007_task_kofi _006/dance_event/berlin-dancemode-clone/frontend/src/pages/events.svelte",
      "importPath": "../src/pages/events.svelte",
      "isLayout": false,
      "isReset": false,
      "isIndex": false,
      "isFallback": false,
      "isPage": true,
      "ownMeta": {},
      "meta": {
        "recursive": true,
        "preload": false,
        "prerender": true
      },
      "path": "/events",
      "id": "_events",
      "component": () => import('../src/pages/events.svelte').then(m => m.default)
    },
    {
      "isFile": true,
      "isDir": false,
      "file": "index.svelte",
      "filepath": "/index.svelte",
      "name": "index",
      "ext": "svelte",
      "badExt": false,
      "absolutePath": "/Users/safokofi/Desktop/internship/20251007_kofi_task_006_intuivo/20251007_task_kofi _006/dance_event/berlin-dancemode-clone/frontend/src/pages/index.svelte",
      "importPath": "../src/pages/index.svelte",
      "isLayout": false,
      "isReset": false,
      "isIndex": true,
      "isFallback": false,
      "isPage": true,
      "ownMeta": {},
      "meta": {
        "recursive": true,
        "preload": false,
        "prerender": true
      },
      "path": "/index",
      "id": "_index",
      "component": () => import('../src/pages/index.svelte').then(m => m.default)
    }
  ],
  "isLayout": true,
  "isReset": false,
  "isIndex": false,
  "isFallback": false,
  "isPage": false,
  "isFile": true,
  "file": "_layout.svelte",
  "ext": "svelte",
  "badExt": false,
  "importPath": "../src/pages/_layout.svelte",
  "meta": {
    "recursive": true,
    "preload": false,
    "prerender": true
  },
  "path": "/",
  "id": "__layout",
  "component": () => import('../src/pages/_layout.svelte').then(m => m.default)
}


export const {tree, routes} = buildClientTree(_tree)

