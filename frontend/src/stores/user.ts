import { defineStore } from 'pinia';
import type {User} from "@/scripts/definitions";

export const useUserStore = defineStore('user', {
  state: () => ({
    user: null as User | null,
    token: "",
  }),
  getters: {
    isLoggedIn: (state) => !!state.user,
  },
  actions: {
    setUser(user: User, token: string) {
      this.user = user;
      this.token = token;

      localStorage.setItem('user', JSON.stringify(this.user));
      localStorage.setItem('token', token);
    },
    logout() {
      this.user = null;
      this.token = "";

      localStorage.removeItem('user');
      localStorage.removeItem('token');
    },
    init() {
      const storedUser = localStorage.getItem('user');
      const token = localStorage.getItem('token');
      if (storedUser && token) {
        this.setUser(JSON.parse(storedUser) as User, token);
      }
    },
  },
});
