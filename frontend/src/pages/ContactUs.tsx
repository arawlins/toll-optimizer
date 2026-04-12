import { 
  Bot, 
  FileText,
} from 'lucide-react';
import { Navbar } from '../components/Navbar';
import { Footer } from '../components/Footer';

/**
 * Toll Optimizer - Contact Us Page
 * 
 * A clean, editorial contact page aligned with the 'Equilibrium Finance' aesthetic.
 * Features simplified direct contact channels for Support and Press.
 */
export function ContactUs() {
  return (
    <div className="min-h-screen bg-white font-manrope text-[#1e293b] antialiased">
      {/* Navigation Header */}
      <Navbar />

      <main className="max-w-7xl mx-auto px-6 py-16 md:py-32">
        {/* Hero Section */}
        <div className="max-w-3xl mb-20">
          <h1 className="text-6xl md:text-7xl font-extrabold tracking-tight text-slate-900 mb-8 font-headline leading-tight">
            Get in touch with our team
          </h1>
          <p className="text-xl text-slate-500 leading-relaxed max-w-2xl">
            Precision support for toll optimization.
          </p>
        </div>

        {/* Contact Channels */}
        <div className="grid gap-6">
          {/* Support Channel */}
          <div className="group flex flex-col md:flex-row items-start md:items-center gap-8 p-8 md:p-12 bg-slate-50 rounded-[32px] border border-slate-100 transition-all hover:bg-slate-100/50">
            <div className="w-16 h-16 rounded-2xl bg-white flex items-center justify-center text-blue-600 shadow-sm shrink-0">
              <Bot className="w-8 h-8" />
            </div>
            <div>
              <h2 className="text-2xl font-bold text-slate-900 mb-1 font-headline">Support</h2>
              <a href="mailto:support@tolloptimizer.com" className="text-xl font-bold text-blue-600 hover:underline">
                support@tolloptimizer.com
              </a>
              <p className="mt-2 text-slate-500 font-medium">
                Technical assistance and account management inquiries.
              </p>
            </div>
          </div>

          {/* Press Channel */}
          <div className="group flex flex-col md:flex-row items-start md:items-center gap-8 p-8 md:p-12 bg-slate-50 rounded-[32px] border border-slate-100 transition-all hover:bg-slate-100/50">
            <div className="w-16 h-16 rounded-2xl bg-white flex items-center justify-center text-blue-600 shadow-sm shrink-0">
              <FileText className="w-8 h-8" />
            </div>
            <div>
              <h2 className="text-2xl font-bold text-slate-900 mb-1 font-headline">Press</h2>
              <a href="mailto:press@tolloptimizer.com" className="text-xl font-bold text-blue-600 hover:underline">
                press@tolloptimizer.com
              </a>
              <p className="mt-2 text-slate-500 font-medium">
                Media kits, interview requests, and brand partnerships.
              </p>
            </div>
          </div>
        </div>
      </main>

      <Footer />
    </div>
  );
}