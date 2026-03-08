import React from 'react';
import { Navigate } from 'react-router-dom';
import { useAuthStore } from '../store';

export function AuthGuard({ children }: { children: React.ReactNode }) {
  const { token } = useAuthStore();

  if (!token) {
    return <Navigate to="/login" replace />;
  }

  return <>{children}</>;
}
