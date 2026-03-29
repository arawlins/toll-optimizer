import React, { useState } from 'react';
import { useQuery } from '@tanstack/react-query';
import { endpoints } from '../lib/api';
import type { AnalysisResponse } from '../lib/api';
import { UploadDropzone } from '../components/UploadDropzone';

import { History as HistoryIcon, TrendingDown, Clock, MapPin, ChevronDown, ChevronUp, Route } from 'lucide-react';
import { Navbar } from '../components/Navbar';
import clsx from 'clsx';

export function Dashboard() {

  const [analysis, setAnalysis] = useState<AnalysisResponse | null>(null);
  const [viewMode, setViewMode] = useState<'time' | 'distance'>('time');
  const [expandedCentroids, setExpandedCentroids] = useState<string[]>([]);
  const [expandedTransponders, setExpandedTransponders] = useState<string[]>([]);

  const toggleCentroid = (id: string, e?: React.MouseEvent) => {
    e?.stopPropagation();
    setExpandedCentroids(prev =>
      prev.includes(id) ? prev.filter(i => i !== id) : [...prev, id]
    );
  };

  const toggleTransponder = (plate: string) => {
    setExpandedTransponders(prev =>
      prev.includes(plate) ? prev.filter(p => p !== plate) : [...prev, plate]
    );
  };

  const { data: historyData, refetch: refetchHistory } = useQuery({
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

  const groupedAnalysis = (Array.isArray(currentAnalysis) ? currentAnalysis : []).reduce((acc: any, curr: any) => {
    const plate = curr.transponder_plate;
    if (!acc[plate]) {
      acc[plate] = {
        plate,
        summaries: [],
        totalSavings: 0,
        totalCentroids: 0
      };
    }
    acc[plate].summaries.push(curr);
    const centroids = Array.isArray(curr.centroids) ? curr.centroids : [];
    const savings = centroids.reduce((sum: number, c: any) => sum + (c.total_optimized_savings || 0), 0) || 0;
    acc[plate].totalSavings += savings;

    // Filter centroids consistently with the expanded view
    const centroidsToCount = viewMode === 'time'
      ? centroids.filter((c: any) => c.total_optimized_savings > 0.005)
      : centroids;

    // Only count centroids that actually contain trips
    acc[plate].totalCentroids += centroidsToCount.filter((c: any) => Array.isArray(c.trips) && c.trips.length > 0).length;
    return acc;
  }, {});

  const transponders = Object.values(groupedAnalysis);

  const getTripCost = (trip: any) => {
    if (!trip) return 0;
    return (Number(trip.toll_charge) || 0) + (Number(trip.trip_toll_charge) || 0) + (Number(trip.camera_charge) || 0);
  };

  const renderTripsTable = (trips: any[], title?: string) => {
    if (trips.length === 0) return null;

    const firstTrip = trips[0];

    return (
      <div className="space-y-2">
        {title && <h4 className="px-4 text-[10px] font-bold text-gray-400 uppercase tracking-widest">{title}</h4>}
        <div className="overflow-hidden rounded-xl border border-gray-200 shadow-sm">
          <div className="overflow-x-auto">
            <table className="min-w-full divide-y divide-gray-200 text-xs">
              <thead className="bg-gray-50">
                <tr>
                  <th className="px-4 py-2.5 text-left text-gray-500 font-bold uppercase tracking-wider">Date</th>
                  <th className="px-4 py-2.5 text-left text-gray-500 font-bold uppercase tracking-wider">Time</th>
                  <th className="px-4 py-2.5 text-left text-gray-500 font-bold uppercase tracking-wider">Entry/Exit</th>
                  <th className="px-4 py-2.5 text-left text-gray-500 font-bold uppercase tracking-wider">Cost</th>
                  {viewMode === 'distance' && (
                    <th className="px-4 py-2.5 text-left text-gray-500 font-bold uppercase tracking-wider">
                      Suggested Trip
                    </th>
                  )}
                  {viewMode === 'time' && (
                    <>
                      <th className="px-4 py-2.5 text-left text-gray-500 font-bold uppercase tracking-wider">
                        Before {firstTrip.prev_timeslot_target || ''}
                      </th>
                      <th className="px-4 py-2.5 text-left text-gray-500 font-bold uppercase tracking-wider">
                        After {firstTrip.next_timeslot_target || ''}
                      </th>
                    </>
                  )}
                  {viewMode === 'distance' && (
                    <th className="px-4 py-2.5 text-left text-gray-500 font-bold uppercase tracking-wider">
                      Suggested Cost
                    </th>
                  )}
                </tr>
              </thead>
              <tbody className="bg-white divide-y divide-gray-100">
                {trips.map((ts: any, tIdx: number) => {
                  const actualCost = getTripCost(ts.trip);
                  return (
                    <tr key={tIdx} className="hover:bg-blue-50/30 transition-colors">
                      <td className="px-4 py-2.5 whitespace-nowrap font-medium">{ts.trip?.date_of_trip}</td>
                      <td className="px-4 py-2.5 whitespace-nowrap">{ts.trip?.entry_time}</td>
                      <td className="px-4 py-2.5 whitespace-nowrap max-w-[150px] truncate font-medium text-gray-600" title={`${ts.trip?.entry_point} → ${ts.trip?.exit_point}`}>
                        {ts.trip?.entry_point} → {ts.trip?.exit_point}
                      </td>
                      <td className="px-4 py-2.5 whitespace-nowrap font-bold text-gray-900">${actualCost.toFixed(2)}</td>
                      {viewMode === 'distance' && (
                        <td className={clsx(
                          "px-4 py-2.5 whitespace-nowrap max-w-[150px] truncate font-medium",
                          ts.optimized_entry ? "text-blue-600" : "text-gray-400"
                        )} title={ts.optimized_entry ? `${ts.optimized_entry} → ${ts.optimized_exit}` : ''}>
                          {ts.optimized_entry ? `${ts.optimized_entry} → ${ts.optimized_exit}` : ' - '}
                        </td>
                      )}
                      {viewMode === 'time' && (
                        <>
                          <td className={clsx(
                            "px-4 py-2.5 whitespace-nowrap font-medium",
                            ts.total_cost_previous_timeslot !== null && ts.total_cost_previous_timeslot < actualCost - 0.005 ? "text-green-600 font-bold" : "text-gray-400"
                          )}>
                            {ts.total_cost_previous_timeslot !== null ? `$${Number(ts.total_cost_previous_timeslot).toFixed(2)}` : '-'}
                          </td>
                          <td className={clsx(
                            "px-4 py-2.5 whitespace-nowrap font-medium",
                            ts.total_cost_next_timeslot !== null && ts.total_cost_next_timeslot < actualCost - 0.005 ? "text-green-600 font-bold" : "text-gray-400"
                          )}>
                            {ts.total_cost_next_timeslot !== null ? `$${Number(ts.total_cost_next_timeslot).toFixed(2)}` : '-'}
                          </td>
                        </>
                      )}
                      {viewMode === 'distance' && (
                        <td className={clsx(
                          "px-4 py-2.5 whitespace-nowrap font-medium",
                          ts.optimized_cost !== null && ts.optimized_cost < actualCost - 0.005 ? "text-green-600 font-bold" : "text-gray-400"
                        )}>
                          {ts.optimized_cost !== null ? `$${Number(ts.optimized_cost).toFixed(2)}` : '-'}
                        </td>
                      )}
                    </tr>
                  );
                })}
              </tbody>
            </table>
          </div>
        </div>
      </div>
    );
  };

  return (
    <div className="min-h-screen bg-gray-50 text-gray-900">
      {/* Header */}
      <Navbar />

      <main className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8 space-y-8">

        {/* Upload Section */}
        <section className="bg-white rounded-2xl shadow-sm border border-gray-100 overflow-hidden">
          <div className="p-6">
            <h2 className="text-lg font-semibold mb-4">Analyze New Statement</h2>
            <UploadDropzone onSuccess={handleUploadSuccess} />
          </div>
        </section>

        {/* Results Section */}
        {analysis && (
          <section className="bg-white rounded-2xl shadow-sm border border-gray-100 overflow-hidden animate-in fade-in slide-in-from-bottom-4 duration-500">
            <div className="p-6 border-b border-gray-100 flex flex-col md:flex-row justify-between items-start md:items-center gap-4">
              <div className="flex items-center gap-2">
                <TrendingDown className="w-6 h-6 text-green-600" />
                <h2 className="text-xl font-bold">Usage Analysis</h2>
              </div>

              <div className="flex bg-gray-100 p-1 rounded-xl">
                <button
                  onClick={() => setViewMode('time')}
                  className={clsx(
                    "px-5 py-2 rounded-lg text-sm font-semibold transition-all",
                    viewMode === 'time' 
                      ? "bg-white shadow-sm text-blue-600" 
                      : "text-gray-500 hover:text-gray-700"
                  )}
                >
                  Time-Based
                </button>
                <button
                  onClick={() => setViewMode('distance')}
                  className={clsx(
                    "px-5 py-2 rounded-lg text-sm font-semibold transition-all",
                    viewMode === 'distance' 
                      ? "bg-white shadow-sm text-blue-600" 
                      : "text-gray-500 hover:text-gray-700"
                  )}
                >
                  Distance-Based
                </button>
              </div>
            </div>

            <div className="p-6">
              <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4 mb-8">
                <div className="bg-blue-50/50 p-4 rounded-2xl border border-blue-100">
                  <p className="text-xs text-blue-600 font-bold uppercase tracking-wider mb-1">Total Trips</p>
                  <p className="text-3xl font-black text-blue-900">{analysis.total_trips}</p>
                </div>
                <div className="bg-slate-50 p-4 rounded-2xl border border-slate-100">
                  <p className="text-xs text-slate-500 font-bold uppercase tracking-wider mb-1">Actual Total Cost</p>
                  <p className="text-3xl font-black text-slate-900">${analysis.total_cost.toFixed(2)}</p>
                </div>
                <div className="bg-green-50/50 p-4 rounded-2xl border border-green-100">
                  <p className="text-xs text-green-600 font-bold uppercase tracking-wider mb-1">Time Savings</p>
                  <p className="text-3xl font-black text-green-700">${analysis.time_based_savings.toFixed(2)}</p>
                </div>
                <div className="bg-purple-50/50 p-4 rounded-2xl border border-purple-100">
                  <p className="text-xs text-purple-600 font-bold uppercase tracking-wider mb-1">Distance Savings</p>
                  <p className="text-3xl font-black text-purple-700">${analysis.distance_based_savings.toFixed(2)}</p>
                </div>
              </div>

                            <div className="space-y-6">
                {(transponders as any[]).map((transponder) => {
                  const isTransponderExpanded = expandedTransponders.includes(transponder.plate);

                  return (
                    <div key={transponder.plate} className="border border-gray-100 rounded-2xl bg-white shadow-sm hover:border-blue-200 transition-all overflow-hidden">
                      {/* Transponder Header (Collapsible) */}
                      <button 
                        onClick={() => toggleTransponder(transponder.plate)}
                        className="w-full text-left p-5 flex justify-between items-center hover:bg-gray-50 transition-colors"
                      >
                        <div className="flex items-center gap-4">
                          <div className="bg-blue-100 p-2 rounded-xl">
                            <Route className="w-5 h-5 text-blue-600" />
                          </div>
                          <div>
                            <div className="flex items-center gap-2 mb-0.5">
                              <h3 className="font-bold text-lg text-gray-900">{transponder.plate}</h3>
                              <span className="px-2 py-0.5 bg-gray-100 text-gray-600 text-[10px] font-bold rounded uppercase tracking-widest">
                                Transponder
                              </span>
                            </div>
                            <p className="text-xs text-gray-500 font-medium">
                              {transponder.totalCentroids} common {viewMode === 'time' ? 'times' : 'distances'} found
                            </p>
                          </div>
                        </div>
                        
                        <div className="flex items-center gap-6">
                          <div className="text-right">
                            <p className="text-[10px] font-bold text-gray-400 uppercase tracking-widest mb-0.5">Potential Savings</p>
                            <p className={clsx("text-lg font-black", transponder.totalSavings > 0.005 ? "text-green-600" : "text-gray-400")}>
                              ${Number(transponder.totalSavings).toFixed(2)}
                            </p>
                          </div>
                          <div className="p-2 bg-gray-100 rounded-full text-gray-400 group-hover:bg-blue-100 group-hover:text-blue-600 transition-all">
                            {isTransponderExpanded ? <ChevronUp className="w-5 h-5" /> : <ChevronDown className="w-5 h-5" />}
                          </div>
                        </div>
                      </button>

                      {isTransponderExpanded && (
                        <div className="p-5 pt-0 border-t border-gray-50 animate-in slide-in-from-top-2 duration-200">
                          <div className="grid grid-cols-1 gap-8 mt-6">
                            {transponder.summaries.map((summary: any) => {
                              const centroidsToDisplay = viewMode === 'time'
                                ? (summary.centroids || []).filter((c: any) => c.total_optimized_savings > 0.005)
                                : (summary.centroids || []);

                              const hasSavings = centroidsToDisplay.length > 0;

                              return (
                                <div key={`${transponder.plate}-${summary.direction}`} className="space-y-4">
                                  <div className="flex items-center gap-2 pb-2 border-b border-gray-100">
                                    <MapPin className="w-4 h-4 text-blue-500" />
                                    <h4 className="font-bold text-gray-800 uppercase text-xs tracking-wider">
                                      {summary.direction} Trips
                                    </h4>
                                  </div>

                                  {!hasSavings && viewMode === 'time' ? (
                                    <div className="py-4 px-5 text-center bg-green-50/50 rounded-xl border border-green-100/50">
                                      <p className="text-green-700 text-sm font-bold flex items-center justify-center gap-2">
                                        <TrendingDown className="w-4 h-4" />
                                        Optimal travel times detected for this direction!
                                      </p>
                                    </div>
                                  ) : (
                                    <div className="grid grid-cols-1 gap-4">
                                      {centroidsToDisplay.map((centroid: any, cIdx: number) => {
                                        const centroidId = `${transponder.plate}-${summary.direction}-${cIdx}-${viewMode}`;
                                        const isExpanded = expandedCentroids.includes(centroidId);

                                        return (
                                          <div key={centroidId} className="bg-gray-50/50 rounded-xl p-4 border border-gray-100 hover:bg-gray-50 transition-colors">
                                            <div className="flex flex-wrap justify-between items-center gap-3 mb-3">
                                              <div className="flex items-center gap-2">
                                                <div className={clsx("p-1.5 rounded-lg", viewMode === 'time' ? "bg-blue-100" : "bg-purple-100")}>
                                                  {viewMode === 'time' ? (
                                                    <Clock className="w-4 h-4 text-blue-600" />
                                                  ) : (
                                                    <Route className="w-4 h-4 text-purple-600" />
                                                  )}
                                                </div>
                                                <p className="font-bold text-gray-800 text-sm">
                                                  {viewMode === 'time' 
                                                    ? `Trips near ${centroid.centroid_time}` 
                                                    : `${centroid.representative_entry || 'Entry'} -> ${centroid.representative_exit || 'Exit'}`
                                                  }
                                                </p>
                                              </div>
                                              <button
                                                onClick={(e) => toggleCentroid(centroidId, e)}
                                                className="flex items-center gap-1.5 text-xs bg-white px-3 py-1.5 rounded-lg border border-gray-200 text-blue-600 hover:bg-blue-50 hover:border-blue-200 transition-all font-bold shadow-sm"
                                              >
                                                {centroid.trips?.length || 0} trips 
                                                {isExpanded ? <ChevronUp className="w-3 h-3" /> : <ChevronDown className="w-3 h-3" />}
                                              </button>
                                            </div>

                                            {centroid.optimization_advice && (
                                              <div className="mb-4 p-3 bg-green-100/50 rounded-xl text-xs text-green-800 border border-green-200/50 font-bold flex items-start gap-2">
                                                <TrendingDown className="w-4 h-4 flex-shrink-0" />
                                                <span>{centroid.optimization_advice}</span>
                                              </div>
                                            )}

                                            {isExpanded && (
                                              <div className="mt-4 space-y-4 animate-in zoom-in-95 duration-200">
                                                {(() => {
                                                  const weekdayTrips = (centroid.trips || []).filter((ts: any) => ts.trip?.day_type === 'Weekday');
                                                  const weekendTrips = (centroid.trips || []).filter((ts: any) => ts.trip?.day_type === 'Weekend' || ts.trip?.day_type === 'Holiday');
                                                  
                                                  return (
                                                    <>
                                                      {renderTripsTable(weekdayTrips, "Weekday Trips")}
                                                      {renderTripsTable(weekendTrips, "Weekend/Holiday Trips")}
                                                    </>
                                                  );
                                                })()}
                                              </div>
                                            )}

                                            <div className="flex justify-between items-center mt-3 pt-3 border-t border-gray-200/60">
                                              <span className="text-[10px] text-gray-500 font-medium">
                                                {viewMode === 'time'
                                                  ? `Average entry time: ${centroid.average_entry_time}`
                                                  : `Average distance: ${Number(centroid.average_distance).toFixed(2)} km`}
                                              </span>
                                              <div className="flex items-center gap-1.5">
                                                <span className="text-[10px] font-bold text-gray-400 uppercase tracking-tight">Potential</span>
                                                <span className="text-sm font-black text-green-600">
                                                  Save ${Number(centroid.total_optimized_savings).toFixed(2)}
                                                </span>
                                              </div>
                                            </div>
                                          </div>
                                        );
                                      })}
                                    </div>
                                  )}
                                </div>
                              );
                            })}
                          </div>
                        </div>
                      )}
                    </div>
                  );
                })}
              </div>
            </div>
          </section>
        )}

        {/* History Section */}
        <section className="bg-white rounded-2xl shadow-sm border border-gray-100 overflow-hidden">
          <div className="p-6 border-b border-gray-100">
            <div className="flex items-center gap-2">
              <HistoryIcon className="w-6 h-6 text-blue-600" />
              <h2 className="text-xl font-bold">Upload History</h2>
            </div>
          </div>

          <div className="overflow-x-auto">
            <table className="min-w-full divide-y divide-gray-200">
              <thead className="bg-gray-50">
                <tr>
                  <th className="px-6 py-4 text-left text-xs font-bold text-gray-500 uppercase tracking-widest">Date</th>
                  <th className="px-6 py-4 text-left text-xs font-bold text-gray-500 uppercase tracking-widest">File</th>
                  <th className="px-6 py-4 text-left text-xs font-bold text-gray-500 uppercase tracking-widest">Trips</th>
                  <th className="px-6 py-4 text-left text-xs font-bold text-gray-500 uppercase tracking-widest">Actual Cost</th>
                  <th className="px-6 py-4 text-left text-xs font-bold text-gray-500 uppercase tracking-widest">Max Savings</th>
                  <th className="px-6 py-4 text-left text-xs font-bold text-gray-500 uppercase tracking-widest">Savings %</th>
                </tr>
              </thead>
              <tbody className="bg-white divide-y divide-gray-100">
                {Array.isArray(historyData) && historyData.map((item) => {
                  const savingsPercent = item.cost_actual > 0
                    ? (item.savings / item.cost_actual) * 100
                    : 0;

                  return (
                    <tr key={item.id} className="hover:bg-gray-50 transition-colors">
                      <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-500 font-medium">
                        {new Date(item.uploaded_at).toLocaleDateString()}
                      </td>
                      <td className="px-6 py-4 whitespace-nowrap text-sm font-bold text-gray-900">
                        {item.filename}
                      </td>
                      <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-600">
                        {item.total_trips}
                      </td>
                      <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-600 font-medium">
                        ${Number(item.cost_actual).toFixed(2)}
                      </td>
                      <td className="px-6 py-4 whitespace-nowrap text-sm font-black text-green-600">
                        ${Number(item.savings).toFixed(2)}
                      </td>
                      <td className="px-6 py-4 whitespace-nowrap text-sm text-blue-600 font-black">
                        {savingsPercent.toFixed(1)}%
                      </td>
                    </tr>
                  );
                })}
                {(!Array.isArray(historyData) || historyData.length === 0) && (
                  <tr>
                    <td colSpan={6} className="px-6 py-12 text-center text-sm text-gray-400 font-medium">
                      No history found. Upload your first statement to get started!
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