import {createApp} from "vue";
import './assets/base.css'
import PrimeVue from "primevue/config";
import StyleClass from 'primevue/styleclass';
import App from "./App.vue";
import ToastService from 'primevue/toastservice';

import "primeicons/primeicons.css";
import {createRouter, createWebHistory} from "vue-router";
import {requestBackend} from "@/scripts/data";
import {useUserStore} from "@/stores/user";
import Login from "@/views/Login.vue";
import {createPinia} from "pinia";
import Homepage from "@/views/Homepage.vue";
import {IntelligencePreset} from "@/theme";
import Blog from "@/views/Blog.vue";
import Team from "@/views/Team.vue";

const pinia = createPinia()
const app = createApp(App);

app.use(PrimeVue, {
  theme: {
    preset: IntelligencePreset,
    options: {
      darkModeSelector: true,
    },
    },
});
app.use(ToastService);

app.use(pinia)
app.directive('styleclass', StyleClass);

const routes = [
  {name: "Homepage", path: '/', component: Homepage},
  {path: '/blog', component: Blog, meta: {needsAuth: false}},
  {path: '/team', component: Team, meta: {needsAuth: false}},
  {
    name: "Login",
    path: '/login',
    component: Login,
    meta: {layout: 'empty'}
  },
];

const router = createRouter({
  history: createWebHistory(),
  routes,
})

router.beforeEach(async (to, from) => {
  const userStore = useUserStore();

  const token = userStore.token;

  let isAuthenticated = true;

  if (!token) {
    console.log("Redirecting to login")
    isAuthenticated = false;
  } else {
    try {
      const response = await requestBackend("GET", "auth/me", null, token);
      console.log("Response: " + response)
    } catch (error) {
      console.log("Redirecting to login wrong token")
      userStore.logout();
      isAuthenticated = false;
    }
  }

  console.log("current token: " + token)
  if (to.name !== 'Login') {
    if (!isAuthenticated && to.meta.needsAuth) {
      return {name: 'Login'}
    }
  } else {
    if (isAuthenticated) {
      return {name: 'Dashboard'}
    }
  }
})


app.use(router);

useUserStore().init();

app.mount("#app");
