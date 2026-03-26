import { useNavigate } from 'react-router-dom';
import { TrendingDown, FileText, ShieldCheck, ArrowRight, Wallet, LogIn } from 'lucide-react';

export function Landing() {
  const navigate = useNavigate();

  return (
    <div className="min-h-screen bg-gray-50 text-gray-900 font-sans">
      {/* Navigation */}
      <nav className="bg-white border-b border-gray-200 sticky top-0 z-10">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-4 flex justify-between items-center">
          <div className="flex items-center gap-2">
            <div className="bg-blue-600 p-1.5 rounded-lg">
              <TrendingDown className="w-5 h-5 text-white" />
            </div>
            <span className="text-xl font-bold tracking-tight">Toll Optimizer</span>
          </div>
          <button
            onClick={() => navigate('/login')}
            className="flex items-center gap-2 px-4 py-2 text-sm font-bold text-blue-600 hover:bg-blue-50 rounded-xl transition-all"
          >
            <LogIn className="w-4 h-4" />
            Sign In
          </button>
        </div>
      </nav>

      <main>
        {/* Hero Section */}
        <section className="bg-white border-b border-gray-100">
          <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-20 lg:py-32 flex flex-col items-center text-center">
            <div className="inline-flex items-center gap-2 px-3 py-1 bg-blue-50 text-blue-700 rounded-full text-xs font-bold uppercase tracking-wider mb-8 border border-blue-100 animate-in fade-in slide-in-from-bottom-2 duration-500">
              <TrendingDown className="w-3 h-3" />
              Maximize Your Savings
            </div>
            <h1 className="text-5xl lg:text-7xl font-black tracking-tight text-gray-900 mb-8 max-w-4xl animate-in fade-in slide-in-from-bottom-4 duration-700">
              Stop Overpaying for the <span className="text-blue-600">407 ETR</span>
            </h1>
            <p className="text-xl text-gray-500 max-w-2xl mb-12 animate-in fade-in slide-in-from-bottom-6 duration-1000">
              Toll Optimizer analyzes your usage patterns and finds hidden opportunities to save money on every trip.
            </p>
            <div className="flex flex-col sm:flex-row gap-4 animate-in fade-in slide-in-from-bottom-8 duration-1000">
              <button
                onClick={() => navigate('/login')}
                className="flex items-center justify-center gap-2 px-8 py-4 bg-blue-600 text-white rounded-2xl font-bold text-lg hover:bg-blue-700 shadow-lg shadow-blue-200 transition-all group"
              >
                Get Started Now
                <ArrowRight className="w-5 h-5 group-hover:translate-x-1 transition-transform" />
              </button>
            </div>
          </div>
        </section>

        {/* Features Section */}
        <section className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-24">
          <div className="grid grid-cols-1 md:grid-cols-3 gap-12">
            <div className="space-y-6">
              <div className="w-14 h-14 bg-blue-100 rounded-2xl flex items-center justify-center">
                <Wallet className="w-8 h-8 text-blue-600" />
              </div>
              <h3 className="text-2xl font-bold">Save Money</h3>
              <p className="text-gray-500 leading-relaxed">
                This application is meant to help save money on the 407 ETR highway by analyzing your historical usage and identifying cheaper alternatives.
              </p>
            </div>

            <div className="space-y-6">
              <div className="w-14 h-14 bg-green-100 rounded-2xl flex items-center justify-center">
                <FileText className="w-8 h-8 text-green-600" />
              </div>
              <h3 className="text-2xl font-bold">Simple Upload</h3>
              <p className="text-gray-500 leading-relaxed">
                It requires you to have an account on <a href="https://www.407etr.com" target="_blank" rel="noopener noreferrer" className="text-blue-600 hover:underline font-medium">407etr.com</a> so you can download a CSV version of your statement and upload it here.
              </p>
            </div>

            <div className="space-y-6">
              <div className="w-14 h-14 bg-purple-100 rounded-2xl flex items-center justify-center">
                <ShieldCheck className="w-8 h-8 text-purple-600" />
              </div>
              <h3 className="text-2xl font-bold">Privacy First</h3>
              <p className="text-gray-500 leading-relaxed">
                We take your privacy seriously. We only store summary data: filenames, trip counts, total costs, and calculated savings. <strong>Individual trip data is never stored.</strong>
              </p>
            </div>
          </div>
        </section>

        {/* Data Points Section */}
        <section className="bg-gray-100 py-24 border-y border-gray-200">
          <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
            <div className="bg-white rounded-[2.5rem] p-8 lg:p-16 shadow-sm border border-gray-200">
              <div className="grid grid-cols-1 lg:grid-cols-2 gap-16 items-center">
                <div>
                  <h2 className="text-4xl font-black mb-6">Transparency Matters</h2>
                  <p className="text-lg text-gray-500 mb-8">
                    We believe you should know exactly what happens with your data. Our analysis runs locally on your upload, and only the results are saved to your account.
                  </p>
                  <ul className="space-y-4">
                    {[
                      'Filename for your reference',
                      'Total number of trips processed',
                      'The total cost of the statement',
                      'The maximum potential savings'
                    ].map((item, i) => (
                      <li key={i} className="flex items-center gap-3 font-bold text-gray-700">
                        <div className="w-6 h-6 bg-green-100 rounded-full flex items-center justify-center flex-shrink-0">
                          <div className="w-2 h-2 bg-green-600 rounded-full" />
                        </div>
                        {item}
                      </li>
                    ))}
                  </ul>
                </div>
                <div className="grid grid-cols-2 gap-4">
                  <div className="bg-blue-50 p-8 rounded-3xl border border-blue-100 text-center">
                    <p className="text-blue-600 font-bold uppercase tracking-tighter text-xs mb-2">Trips Stored</p>
                    <p className="text-5xl font-black text-blue-900">0</p>
                  </div>
                  <div className="bg-green-50 p-8 rounded-3xl border border-green-100 text-center">
                    <p className="text-green-600 font-bold uppercase tracking-tighter text-xs mb-2">Analysis</p>
                    <p className="text-5xl font-black text-green-900">100%</p>
                  </div>
                  <div className="bg-purple-50 p-8 rounded-3xl border border-purple-100 col-span-2 text-center">
                    <p className="text-purple-600 font-bold uppercase tracking-tighter text-xs mb-2">Your Privacy</p>
                    <p className="text-3xl font-black text-purple-900">Protected</p>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </section>
      </main>

      <footer className="bg-white py-12 border-t border-gray-100">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 text-center text-gray-400 text-sm font-medium">
          © {new Date().getFullYear()} Toll Optimizer. Not affiliated with 407 ETR Concession Company Limited.
        </div>
      </footer>
    </div>
  );
}