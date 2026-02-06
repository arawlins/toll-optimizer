import { useState } from 'react';
import { useQuery } from '@tanstack/react-query';
import { endpoints, AnalysisResult } from '../lib/api';
import { UploadDropzone } from '../components/UploadDropzone';
import { useAuthStore } from '../store';
import { LogOut, History, TrendingDown } from 'lucide-react';

export function Dashboard() {
  const { user, logout } = useAuthStore();
  const [analysis, setAnalysis] = useState<AnalysisResult[] | null>(null);

  const { data: history, refetch: refetchHistory } = useQuery({
    queryKey: ['history'],
    queryFn: async () => {
      const res = await endpoints.history();
      return res.data;
    },
  });

  const handleUploadSuccess = (data: AnalysisResult[]) => {
    setAnalysis(data);
    refetchHistory();
  };

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
            <div className="flex items-center gap-2 mb-6">
              <TrendingDown className="w-6 h-6 text-green-600" />
              <h2 className="text-lg font-semibold text-gray-900">Optimization Analysis</h2>
            </div>
            
            <div className="space-y-6">
              {analysis.map((summary, idx) => (
                <div key={idx} className="border rounded-lg p-4">
                  <div className="flex justify-between items-start mb-4">
                    <div>
                      <h3 className="font-medium text-gray-900">
                        {summary.transponder_plate}
                      </h3>
                      <p className="text-sm text-gray-500">{summary.direction}</p>
                    </div>
                  </div>
                  
                  <div className="space-y-2">
                    {summary.centroids.map((centroid: any, cIdx: number) => (
                      <div key={cIdx} className="text-sm border-l-2 border-blue-200 pl-3 py-1">
                        <p className="font-medium text-gray-700">
                          Approx {centroid.centroid_time}
                        </p>
                        <p className="text-gray-600">
                          {centroid.trips.length} trips • Potential Savings: 
                          <span className="text-green-600 font-bold ml-1">
                            ${centroid.total_optimized_savings.toFixed(2)}
                          </span>
                        </p>
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
                  <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">Potential Savings</th>
                </tr>
              </thead>
              <tbody className="bg-white divide-y divide-gray-200">
                {history?.map((item) => (
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
                      ${item.cost_actual}
                    </td>
                    <td className="px-6 py-4 whitespace-nowrap text-sm font-bold text-green-600">
                      ${item.savings}
                    </td>
                  </tr>
                ))}
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
