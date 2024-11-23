import {useUserStore} from "@/stores/user";
import {useRouter} from "vue-router";
const router = useRouter();

export async function logout() {
  useUserStore().logout();

  await router.push('/login');
}

export function isEmptyOrWhiteSpace(str: string | undefined) {
  return !str || !str.trim();
}