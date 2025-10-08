import { Router } from '@roxi/routify'
import { routes } from '../.routify/routes'

export default new Router({
  routes,
  urlRewrite: [
    {
      toInternal: [/^\/(.*)/, '/$1'],
      toExternal: [/^\/(.*)/, '/$1']
    }
  ]
})