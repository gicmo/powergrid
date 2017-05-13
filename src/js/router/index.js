import Vue from 'vue'
import Router from 'vue-router'

import RunTable from '@/js/components/RunTable'

Vue.use(Router)

export default new Router({
  routes: [
    {
      path: '/',
      name: 'Overview',
      component: RunTable
    }
  ]
})
