import {createApp} from "vue";
import './assets/base.css'
import PrimeVue from "primevue/config";
import StyleClass from 'primevue/styleclass';
import App from "./App.vue";
import ToastService from 'primevue/toastservice';

import "primeicons/primeicons.css";
import Aura from "@primevue/themes/aura";
import {createRouter, createWebHistory} from "vue-router";
import {requestBackend} from "@/scripts/data";
import {useUserStore} from "@/stores/user";
import Login from "@/views/Login.vue";
import {createPinia} from "pinia";
import {definePreset} from "@primevue/themes";
import Users from "@/views/Users.vue";
import MyProfile from "@/views/MyProfile.vue";
import Homepage from "@/views/Homepage.vue";

const pinia = createPinia()
const app = createApp(App);

const MyPreset = definePreset(Aura, {
  semantic: {
    primary: {
      50: '{violet.50}',
      100: '{violet.100}',
      200: '{violet.200}',
      300: '{violet.300}',
      400: '{violet.400}',
      500: '{violet.500}',
      600: '{violet.600}',
      700: '{violet.700}',
      800: '{violet.800}',
      900: '{violet.900}',
      950: '{violet.950}'
    },
    colorScheme: {
      light: {
        formField: {
          background: '{surface.100}',
          disabledBackground: '{surface.200}',
        }
      },
      dark: {
        formField: {
          background: '{surface.800}',
          disabledBackground: '{surface.200}',
        },
        content: {
          background: '{surface.900}',
          hoverBackground: '{surface.800}',
          borderColor: '{surface.700}',
          color: '{text.color}',
          hoverColor: '{text.hover.color}'
        },
      }
    }
  }
});

app.use(PrimeVue, {
  theme: {
    preset: MyPreset,
    options: {
      darkModeSelector: '.idark',
    },
    },
});
app.use(ToastService);

app.use(pinia)
app.directive('styleclass', StyleClass);

const routes = [
  {name: "Homepage", path: '/', component: Homepage},
  {path: '/myprofile', component: MyProfile, meta: {needsAuth: true}},
  {path: '/users', component: Users, meta: {needsAuth: true}},
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
