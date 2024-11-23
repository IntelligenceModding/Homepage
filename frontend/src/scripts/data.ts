import {useUserStore} from '@/stores/user';
import type {User} from "@/scripts/definitions";

export async function requestBackend(method: 'GET' | 'POST' | 'PATCH' | 'PUT' | 'DELETE', endpoint: string, body?: any, token?: string | null) {
  const headers: { [key: string]: string } = {
    'Content-Type': 'application/json; charset=UTF-8',
  };

  if (token) {
    headers['Authorization'] = `Bearer ${token}`;
  }

  const options: RequestInit = {
    method,
    headers,
    ...(body && {body: JSON.stringify(body)}),
  };

  return await fetch(`/api/v1/${endpoint}`, options);
}

export async function putImageBackend(endpoint: string, files?: any, token?: string | null) {
  const headers: { [key: string]: string } = {
  };

  if (token) {
    headers['Authorization'] = `Bearer ${token}`;
  }

  const options: RequestInit = {
    method: 'PUT',
    headers,
    body: files
  };

  return await fetch(`/api/v1/${endpoint}`, options);
}

export async function fetchUsers(token: string): Promise<User[]> {
  let data = await (await requestBackend("GET", "users", null, token)).json()

  return mapUser(data);
}

export async function deleteUser(user: User, token: string): Promise<boolean> {
  let code = (await requestBackend("DELETE", `users/${user.id}`, null, token)).status;

  return code == 200;
}

/**
 * Checks if a user exists based on the id, name or email
 */
export async function userExists(user: string, token: string): Promise<boolean> {
  let code = (await requestBackend("GET", `users/exists/${user}`, null, token)).status;

  return code == 200;
}

export async function patchUser(user: User, token: string): Promise<User> {
  let data = await (await requestBackend("PATCH", `users/${user.id}`, user, token)).json()

  return data as User;
}

export async function createUser(user: User, token: string): Promise<boolean> {
  let code = (await requestBackend("POST", `users`, user, token)).status;

  return code == 200;
}

export async function auth(user: string, password: string): Promise<boolean> {
  const body = {
    username: user,
    password: password
  }

  try {
    const response = await requestBackend("POST", "auth/login", body)
    if (response.status === 200) {
      const token = (await response.json()).token;
      const user = await (await requestBackend("GET", "auth/me", null, token)).json() as User
      const userStore = useUserStore();

      userStore.setUser(user, token)
      return true;
    }
  } catch (error) {
    console.log(error)
  }
  return false;
}

function mapUser(jsonData: any[]): User[] {
  return jsonData.map(repo => repo as User);
}