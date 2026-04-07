import axios from 'axios';

if (import.meta.env.VITE_API_URL === undefined) {
  throw new Error("VITE_API_URL environment variable is not set.");
}

const api = axios.create({
  baseURL: import.meta.env.VITE_API_URL,
});

// Add auth interceptor
api.interceptors.request.use((config) => {
  const token = sessionStorage.getItem('token');
  if (token) {
    config.headers.Authorization = `Bearer ${token}`;
  }
  return config;
});

export interface User {
  id: string;
  email: string;
}

export interface AuthResponse {
  token: string;
  user: User;
}

export interface UploadSummary {
  id: string;
  filename: string;
  total_trips: number;
  cost_actual: number;
  cost_optimized: number;
  savings: number;
  uploaded_at: string;
}

export interface TripRecord {
  date_of_trip: string;
  entry_time: string;
  entry_point: string;
  exit_point: string;
  day_type: string;
  toll_charge?: string | number;
  trip_toll_charge?: string | number;
  camera_charge?: string | number;
}

export interface TimeTripSummary {
  trip: TripRecord;
  total_cost_previous_timeslot: number | null;
  total_cost_next_timeslot: number | null;
  prev_timeslot_target: string | null;
  next_timeslot_target: string | null;
}

export interface DistanceTripSummary {
  trip: TripRecord;
  optimized_entry: string | null;
  optimized_exit: string | null;
  optimized_cost: number | null;
}

export interface TimeCentroid {
  centroid_time: string;
  average_entry_time: string;
  total_optimized_savings: number;
  optimization_advice: string | null;
  trips: TimeTripSummary[];
}

export interface DistanceCentroid {
  representative_entry: string | null;
  representative_exit: string | null;
  average_distance: number;
  total_optimized_savings: number;
  optimization_advice: string | null;
  trips: DistanceTripSummary[];
}

export interface TimeAnalysisResult {
  transponder_plate: string;
  direction: string;
  centroids: TimeCentroid[];
}

export interface DistanceAnalysisResult {
  transponder_plate: string;
  direction: string;
  centroids: DistanceCentroid[];
}

export interface AnalysisResponse {
  total_trips: number;
  total_cost: number;
  time_based_savings: number;
  distance_based_savings: number;
  time_analysis: TimeAnalysisResult[];
  distance_analysis: DistanceAnalysisResult[];
}

export const endpoints = {
  login: (data: any) => api.post<AuthResponse>('/auth/login', data),
  register: (data: any) => api.post<AuthResponse>('/auth/register', data),
  history: () => api.get<UploadSummary[]>('/api/history'),
  analyze: (file: File) => {
    const formData = new FormData();
    formData.append('file', file);
    return api.post<AnalysisResponse>('/api/analyze', formData, {
      headers: {
        'Content-Type': 'multipart/form-data',
      },
    });
  },
};

export default api;
