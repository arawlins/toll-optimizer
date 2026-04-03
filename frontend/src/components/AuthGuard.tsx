import React, { useEffect } from 'react';
import { Navigate } from 'react-router-dom';
import { useAuthStore } from '../store';

export function AuthGuard({ children }: { children: React.ReactNode }) {
  const { token, logout } = useAuthStore();

  let isExpired = false;
  if (token) {
    try {
      const payload = JSON.parse(atob(token.split('.')[1]));
      if (payload.exp && payload.exp * 1000 < Date.now()) {
        isExpired = true;
      }
    } catch (e) {
      isExpired = true;
    }
  }

  useEffect(() => {
    if (isExpired) {
      logout();
    }
  }, [isExpired, logout]);

  if (!token || isExpired) {
    return <Navigate to="/login" replace />;
  }

  return <>{children}</>;
}
