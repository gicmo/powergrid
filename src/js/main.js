// The Vue build version to load with the `import` command
// (runtime-only or standalone) has been set in webpack.base.conf with an alias.
import Vue from 'vue'
import VueResource from 'vue-resource'
import Router from 'vue-router'

import App from './App'
import RunTable from '@/js/components/RunTable'

Vue.use(Router)

Vue.use(VueResource)
Vue.config.productionTip = false

const router = new Router({
  routes: [
    {
      path: '/',
      name: 'Overview',
      component: RunTable
    }
  ]
})

/* eslint-disable no-new */
new Vue({
  el: '#app',
  router,
  template: '<App/>',
  components: { App }
})
