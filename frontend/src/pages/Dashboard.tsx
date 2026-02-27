import { useState } from 'react';
import { useQuery } from '@tanstack/react-query';
import { endpoints } from '../lib/api';
import type { AnalysisResponse } from '../lib/api';
import { UploadDropzone } from '../components/UploadDropzone';
import { useAuthStore } from '../store';
import { LogOut, History, TrendingDown, Clock, MapPin } from 'lucide-react';

export function Dashboard() {
  const { user, logout } = useAuthStore();
  const [analysis, setAnalysis] = useState<AnalysisResponse | null>(null);
  const [viewMode, setViewMode] = useState<'time' | 'distance'>('time');

  const { data: history, refetch: refetchHistory } = useQuery({
    queryKey: ['history'],
    queryFn: async () => {
      const res = await endpoints.history();
      return res.data;
    },
  });

  const handleUploadSuccess = (data: AnalysisResponse) => {
    setAnalysis(data);
    refetchHistory();
  };

  const currentAnalysis = analysis ? (viewMode === 'time' ? analysis.time_analysis : analysis.distance_analysis) : [];

  return (
    <div className="min-h-screen bg-gray-50">
      {/* Header */}
      <header className="bg-white shadow-sm">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-4 flex justify-between items-center">
          <h1 className="text-xl font-bold text-gray-900">Toll Optimizer</h1>
          <div className="flex items-center gap-4">
            <span className="text-sm text-gray-600">{user?.email}</span>
            <button
              onClick={logout}
              className="p-2 text-gray-500 hover:text-red-600 transition-colors"
              title="Logout"
            >
              <LogOut className="w-5 h-5" />
            </button>
          </div>
        </div>
      </header>

      <main className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8 space-y-8">

        {/* Upload Section */}
        <section className="bg-white rounded-xl shadow-sm p-6">
          <h2 className="text-lg font-semibold text-gray-900 mb-4">Analyze New Statement</h2>
          <UploadDropzone onSuccess={handleUploadSuccess} />
        </section>

        {/* Results Section */}
        {analysis && (
          <section className="bg-white rounded-xl shadow-sm p-6 animate-in fade-in slide-in-from-bottom-4">
            <div className="flex flex-col md:flex-row justify-between items-start md:items-center gap-4 mb-6">
              <div className="flex items-center gap-2">
                <TrendingDown className="w-6 h-6 text-green-600" />
                <h2 className="text-lg font-semibold text-gray-900">Optimization Analysis</h2>
              </div>

              <div className="flex bg-gray-100 p-1 rounded-lg">
                <button
                  onClick={() => setViewMode('time')}
                  className={`px-4 py-1.5 rounded-md text-sm font-medium transition-colors ${viewMode === 'time' ? 'bg-white shadow-sm text-blue-600' : 'text-gray-500 hover:text-gray-700'
                    }`}
                >
                  Time-Based
                </button>
                <button
                  onClick={() => setViewMode('distance')}
                  className={`px-4 py-1.5 rounded-md text-sm font-medium transition-colors ${viewMode === 'distance' ? 'bg-white shadow-sm text-blue-600' : 'text-gray-500 hover:text-gray-700'
                    }`}
                >
                  Distance-Based
                </button>
              </div>
            </div>

            <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4 mb-8">
              <div className="bg-blue-50 p-4 rounded-lg">
                <p className="text-xs text-blue-600 font-semibold uppercase">Total Trips</p>
                <p className="text-2xl font-bold text-blue-900">{analysis.total_trips}</p>
              </div>
              <div className="bg-gray-50 p-4 rounded-lg">
                <p className="text-xs text-gray-600 font-semibold uppercase">Actual Total Cost</p>
                <p className="text-2xl font-bold text-gray-900">${analysis.total_cost.toFixed(2)}</p>
              </div>
              <div className="bg-green-50 p-4 rounded-lg">
                <p className="text-xs text-green-600 font-semibold uppercase">Time Savings</p>
                <p className="text-2xl font-bold text-green-900">${analysis.time_based_savings.toFixed(2)}</p>
              </div>
              <div className="bg-purple-50 p-4 rounded-lg">
                <p className="text-xs text-purple-600 font-semibold uppercase">Distance Savings</p>
                <p className="text-2xl font-bold text-purple-900">${analysis.distance_based_savings.toFixed(2)}</p>
              </div>
            </div>

            <div className="space-y-6">
              {currentAnalysis.map((summary, idx) => (
                <div key={idx} className="border rounded-lg p-4 bg-white hover:border-blue-200 transition-colors">
                  <div className="flex justify-between items-start mb-4">
                    <div>
                      <h3 className="font-bold text-gray-900">
                        {summary.transponder_plate}
                      </h3>
                      <p className="text-sm text-gray-500 flex items-center gap-1">
                        <MapPin className="w-3 h-3" /> {summary.direction}
                      </p>
                    </div>
                  </div>

                  <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                    {summary.centroids.map((centroid: any, cIdx: number) => (
                      <div key={cIdx} className="bg-gray-50 rounded-lg p-3 border border-gray-100">
                        <div className="flex justify-between items-start mb-2">
                          <p className="text-sm font-bold text-gray-800 flex items-center gap-1">
                            <Clock className="w-4 h-4 text-blue-500" />
                            {viewMode === 'time' ? `Approx ${centroid.centroid_time}` : `${centroid.centroid_distance.toFixed(1)} km Cluster`}
                          </p>
                          <span className="text-xs bg-white px-2 py-0.5 rounded border text-gray-500">
                            {centroid.trips.length} trips
                          </span>
                        </div>

                        {centroid.optimization_advice && (
                          <div className="mb-2 p-2 bg-green-50 rounded text-sm text-green-800 border border-green-100 font-medium">
                            {centroid.optimization_advice}
                          </div>
                        )}

                        <div className="flex justify-between items-center mt-2 pt-2 border-t border-gray-200">
                          <span className="text-xs text-gray-500">
                            Avg: {viewMode === 'time' ? centroid.average_entry_time : `${centroid.average_distance.toFixed(2)} km`}
                          </span>
                          <span className="text-sm font-bold text-green-600">
                            Save ${centroid.total_optimized_savings.toFixed(2)}
                          </span>
                        </div>
                      </div>
                    ))}
                  </div>
                </div>
              ))}
            </div>
          </section>
        )}

        {/* History Section */}
        <section className="bg-white rounded-xl shadow-sm p-6">
          <div className="flex items-center gap-2 mb-6">
            <History className="w-6 h-6 text-blue-600" />
            <h2 className="text-lg font-semibold text-gray-900">Upload History</h2>
          </div>

          <div className="overflow-x-auto">
            <table className="min-w-full divide-y divide-gray-200">
              <thead>
                <tr>
                  <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">Date</th>
                  <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">File</th>
                  <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">Trips</th>
                  <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">Actual Cost</th>
                  <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">Max Potential Savings</th>
                  <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">Savings %</th>
                </tr>
              </thead>
              <tbody className="bg-white divide-y divide-gray-200">
                {history?.map((item) => {
                  const savingsPercent = item.cost_actual > 0
                    ? (item.savings / item.cost_actual) * 100
                    : 0;

                  return (
                    <tr key={item.id} className="hover:bg-gray-50">
                      <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                        {new Date(item.uploaded_at).toLocaleDateString()}
                      </td>
                      <td className="px-6 py-4 whitespace-nowrap text-sm font-medium text-gray-900">
                        {item.filename}
                      </td>
                      <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                        {item.total_trips}
                      </td>
                      <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                        ${Number(item.cost_actual).toFixed(2)}
                      </td>
                      <td className="px-6 py-4 whitespace-nowrap text-sm font-bold text-green-600">
                        ${Number(item.savings).toFixed(2)}
                      </td>
                      <td className="px-6 py-4 whitespace-nowrap text-sm text-blue-600 font-semibold">
                        {savingsPercent.toFixed(1)}%
                      </td>
                    </tr>
                  );
                })}
                {(!history || history.length === 0) && (
                  <tr>
                    <td colSpan={5} className="px-6 py-8 text-center text-sm text-gray-500">
                      No history found. Upload your first statement!
                    </td>
                  </tr>
                )}
              </tbody>
            </table>
          </div>
        </section>
      </main>
    </div>
  );
}
