import Vue from 'vue'
import Router from 'vue-router'
import Home from './views/Home.vue'
import Bang from './views/Bang.vue'
Vue.use(Router)

export default new Router({
  routes: [
    {
      path: '/',
      name: 'home',
      component: Home
    },
    {
      path: '/bang',
      name: 'bang',
      component: Bang
    },
  ]
})
