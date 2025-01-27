import { RouteName } from './routes';
import { createMemoryHistory, createRouter } from 'vue-router';

const router = createRouter({
  history: createMemoryHistory(),
  routes: [
    {
      path: '/',
      name: RouteName.Library,
      component: () => import('../views/Library.vue')
    },
    {
      path: '/collection',
      name: RouteName.Collection,
      component: () => import('../views/Collection.vue')
    },
    {
      path: '/book-tag',
      name: RouteName.BookTag,
      component: () => import('../views/BookTag.vue')
    }
  ]
});

export { RouteName, router };
