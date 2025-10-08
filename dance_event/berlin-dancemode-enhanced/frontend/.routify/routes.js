
/**
 * @roxi/routify 2.18.18
 * File generated Wed Oct 08 2025 10:20:37 GMT+0200 (Central European Summer Time)
 */

export const __version = "2.18.18"
export const __timestamp = "2025-10-08T08:20:37.557Z"

//buildRoutes
import { buildClientTree } from "@roxi/routify/runtime/buildRoutes.js"

//imports


//options
export const options = {}

//tree
export const _tree = {
  "name": "root",
  "filepath": "/",
  "root": true,
  "ownMeta": {},
  "absolutePath": "src/pages",
  "children": [
    {
      "isFile": false,
      "isDir": true,
      "file": "creator",
      "filepath": "/creator",
      "name": "creator",
      "ext": "",
      "badExt": false,
      "absolutePath": "/Users/safokofi/Desktop/internship/20251007_kofi_task_006_intuivo/20251007_task_kofi _006/dance_event/berlin-dancemode-enhanced/frontend/src/pages/creator",
      "children": [
        {
          "isFile": true,
          "isDir": false,
          "file": "index.svelte",
          "filepath": "/creator/index.svelte",
          "name": "index",
          "ext": "svelte",
          "badExt": false,
          "absolutePath": "/Users/safokofi/Desktop/internship/20251007_kofi_task_006_intuivo/20251007_task_kofi _006/dance_event/berlin-dancemode-enhanced/frontend/src/pages/creator/index.svelte",
          "importPath": "../src/pages/creator/index.svelte",
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
          "path": "/creator/index",
          "id": "_creator_index",
          "component": () => import('../src/pages/creator/index.svelte').then(m => m.default)
        }
      ],
      "isLayout": false,
      "isReset": false,
      "isIndex": false,
      "isFallback": false,
      "isPage": false,
      "ownMeta": {},
      "meta": {
        "recursive": true,
        "preload": false,
        "prerender": true
      },
      "path": "/creator"
    },
    {
      "isFile": true,
      "isDir": false,
      "file": "index.svelte",
      "filepath": "/index.svelte",
      "name": "index",
      "ext": "svelte",
      "badExt": false,
      "absolutePath": "/Users/safokofi/Desktop/internship/20251007_kofi_task_006_intuivo/20251007_task_kofi _006/dance_event/berlin-dancemode-enhanced/frontend/src/pages/index.svelte",
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
    },
    {
      "isFile": true,
      "isDir": false,
      "file": "login.svelte",
      "filepath": "/login.svelte",
      "name": "login",
      "ext": "svelte",
      "badExt": false,
      "absolutePath": "/Users/safokofi/Desktop/internship/20251007_kofi_task_006_intuivo/20251007_task_kofi _006/dance_event/berlin-dancemode-enhanced/frontend/src/pages/login.svelte",
      "importPath": "../src/pages/login.svelte",
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
      "path": "/login",
      "id": "_login",
      "component": () => import('../src/pages/login.svelte').then(m => m.default)
    },
    {
      "isFile": true,
      "isDir": false,
      "file": "register.svelte",
      "filepath": "/register.svelte",
      "name": "register",
      "ext": "svelte",
      "badExt": false,
      "absolutePath": "/Users/safokofi/Desktop/internship/20251007_kofi_task_006_intuivo/20251007_task_kofi _006/dance_event/berlin-dancemode-enhanced/frontend/src/pages/register.svelte",
      "importPath": "../src/pages/register.svelte",
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
      "path": "/register",
      "id": "_register",
      "component": () => import('../src/pages/register.svelte').then(m => m.default)
    }
  ],
  "isLayout": false,
  "isReset": false,
  "isIndex": false,
  "isFallback": false,
  "meta": {
    "recursive": true,
    "preload": false,
    "prerender": true
  },
  "path": "/"
}


export const {tree, routes} = buildClientTree(_tree)

